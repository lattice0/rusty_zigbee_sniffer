use crate::{
    devices::devices::{find_supported_device, UsbDeviceInfo}, SniffError, UsbDataHeader, UsbHeader, UsbTickHeader
};

pub struct CC253X {
    pub channel: u8,
    pub timestamp_tick: u64,
    pub usb_device_info: UsbDeviceInfo,
    pub device_handle: rusb::DeviceHandle<rusb::GlobalContext>,
}

impl CC253X {
    pub fn open(channel: u8) -> Result<Self, SniffError> {
        if let Some(u) = find_supported_device().map_err(|_| SniffError::NoSupportedDevices)? {
            let device = u.device.as_ref().ok_or(SniffError::MissingUsbDevice)?;
            let timeout = std::time::Duration::from_millis(200);
            let usb_device = device.open()?;
            usb_device.kernel_driver_active(0)?;
            usb_device.set_active_configuration(1)?;
            usb_device.claim_interface(0)?;
            let mut buf = [0u8; 256];
            // get identity from firmware command
            usb_device.read_control(0xc0, 192, 0, 0, &mut buf, timeout)?;
            // power on radio, wIndex = 4
            usb_device.write_control(0x40, 197, 0, 4, &[], timeout)?;
            Ok(CC253X {
                channel,
                timestamp_tick: 0,
                usb_device_info: u,
                device_handle: usb_device,
            })
        } else {
            Err(SniffError::Open)
        }
    }

    pub fn blocking_sniff(
        &mut self,
        on_packet: &dyn Fn(&[u8]) -> Result<(), SniffError>,
        on_unknown_packet: Option<&dyn Fn(&[u8]) -> Result<(), SniffError>>,
    ) -> Result<(), SniffError> {
        let mut buf = [0u8; 256];
        let timeout = std::time::Duration::from_millis(10000);
        loop {
            let usb_device = &self.device_handle;
            let res = usb_device.read_bulk(0x83, &mut buf, timeout)?;
            let packet = &buf[0..res];
            if packet.len() < std::mem::size_of::<UsbHeader>()
                || packet.len() < std::mem::size_of::<UsbDataHeader>()
            {
                continue;
            }
            let usb_header = UsbHeader::from(&packet[0..3].try_into()?);
            match usb_header.header_type {
                0 => {
                    let usb_data_header = UsbDataHeader::from(&packet[0..8].try_into()?);
                    if usb_data_header.wpan_length <= 5 {
                        continue;
                    }
                    let max = std::cmp::min(usb_data_header.wpan_length as usize, packet.len());
                    //TODO: fix this
                    if max > 8 {
                        let frame = &packet[8..max];
                        let _rssi = usize::from(*frame.last().ok_or(SniffError::Parse)?);
                        let frame = &frame[0..(frame.len() - 1)];
                        on_packet(frame)?;
                    }
                }
                1 => {
                    let usb_tick_header = UsbTickHeader::from(&packet[0..4].try_into()?);
                    if usb_tick_header.tick == 0x00 {
                        self.timestamp_tick += 0xFFFFFFFF;
                    }
                }
                _ => {
                    if let Some(f) = on_unknown_packet {
                        f(packet)?;
                    }
                }
            }
        }
    }

    pub fn product_name(&self) -> String {
        self.usb_device_info.product_name.to_owned()
    }

    pub fn manufacturer(&self) -> String {
        self.usb_device_info.manufacturer.to_owned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let _cc253x = CC253X::open(11).unwrap();
    }

    //cargo test test_sniff -- --nocapture --ignored
    #[ignore]
    #[test]
    fn test_sniff() {
        let mut cc253x = CC253X::open(15).unwrap();
        let on_packet = |packet: &[u8]| {
            println!("{:?}", packet);
            Ok(())
        };
        let on_unknown_packet = |packet: &[u8]| {
            println!("!unknown packet! {:?}", packet);
            Ok(())
        };
        cc253x.blocking_sniff(&on_packet, Some(&on_unknown_packet)).unwrap();
    }
}
