use std::array::TryFromSliceError;

use crate::{
    devices::devices::{find_supported_device, UsbDeviceInfo},
    UsbDataHeader, UsbHeader, UsbTickHeader,
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
            println!(
                "found device: {:?} from manufacturer {:?} at bus {:03} address {:03}",
                u.product_name,
                u.manufacturer,
                device.bus_number(),
                device.address()
            );
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
        let timeout = std::time::Duration::from_millis(200);
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
}

pub enum SniffError {
    NoSupportedDevices,
    Open,
    Rusb(rusb::Error),
    Parse,
    MissingUsbDevice,
    TryFromSlice
}

impl From<rusb::Error> for SniffError {
    fn from(err: rusb::Error) -> Self {
        SniffError::Rusb(err)
    }
}

impl From<TryFromSliceError> for SniffError {
    fn from(_err: TryFromSliceError) -> Self {
        SniffError::TryFromSlice
    }
}

//wired ekaza button channel 15
//data: [0, 36, 0, 47, 33, c0, 11, 31], frame: [61, 88, 64, c9, d, 28, 17, 57, b, 48, 22, 0, 0, 57, b, 1e, b3, 28, 4c, 23, 0, 0, 8, 47, f3, d8, da, 38, c1, a4, 0, a1, a3, e5, 76, 24, 11, 64, e1, fc, 97], len: 54
