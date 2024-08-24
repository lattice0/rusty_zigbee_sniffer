use rusb::{Device, DeviceList, Error as RusbError, GlobalContext};

pub struct UsbDeviceInfo {
    pub device: Option<Device<GlobalContext>>,
    pub product_name: &'static str,
    pub manufacturer: &'static str,
    pub product_id: u16,
    pub vendor_id: u16
}

// model, manufacturer, vendor_id, product_id
pub const SUPPORTED_DEVICES: [(&str, &str, u16, u16); 1] =
    [("CC2531 Dongle", "Texas Instruments", 0x0451, 0x16ae)];

fn supported_device_match(device: &Device<GlobalContext>) -> Result<Option<UsbDeviceInfo>, RusbError> {
    let descriptor = device.device_descriptor()?;
    let product_id = descriptor.product_id();
    let vendor_id = descriptor.vendor_id();
    Ok(if let Some(d) = SUPPORTED_DEVICES
    .iter()
    .find(|(_, _, vid, pid)| product_id == *pid && vendor_id == *vid) {
        Some(UsbDeviceInfo {
            device: None,
            product_name: d.0,
            manufacturer: d.1,
            vendor_id: d.2,
            product_id: d.3,
        })
    } else {
        None
    })
}

pub fn find_supported_device() -> Result<Option<UsbDeviceInfo>, RusbError> {
    let device_list = DeviceList::new()?;
    for device in device_list.iter() {
        if let Some(d) = supported_device_match(&device)? {
            return Ok(Some(UsbDeviceInfo{
                device: Some(device),
                product_name: d.product_name,
                manufacturer: d.manufacturer,
                vendor_id: d.vendor_id,
                product_id: d.product_id,
            }));
        }
    }
    Ok(None)
}