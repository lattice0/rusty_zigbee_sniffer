pub mod devices;

#[derive(Debug)]
pub struct UsbHeader {
    // 0 for data, 1 for tick
    pub header_type: u8,
    // little endian length
    pub header_len: u16,
}

#[derive(Debug)]
pub struct UsbDataHeader {
    pub header: UsbHeader,
    // little endian timestamp
    pub timestamp: u32,
    pub wpan_length: u8
}

#[derive(Debug)]
pub struct UsbTickHeader
{
	pub header: UsbHeader,
    // tick counter
	pub tick: u8
}

impl From<&[u8; 3]> for UsbHeader {
    fn from(data: &[u8; 3]) -> Self {
        UsbHeader {
            header_type: data[0],
            header_len: u16::from_le_bytes([data[1], 0]),
        }
    }
}

impl From<&[u8; 8]> for UsbDataHeader {
    fn from(data: &[u8; 8]) -> Self {
        UsbDataHeader {
            header: UsbHeader::from(&data[0..3].try_into().unwrap()),
            timestamp: u32::from_le_bytes(data[3..7].try_into().unwrap()),
            wpan_length: data[7],
        }
    }
}

impl From<&[u8; 4]> for UsbTickHeader {
    fn from(data: &[u8; 4]) -> Self {
        UsbTickHeader {
            header: UsbHeader::from(&data[0..3].try_into().unwrap()),
            tick: data[3],
        }
    }
}

#[repr(C)]
pub struct PcapHeader {
    pub magic_number: u32,      // magic number
    pub version_major: u16,     // major version number
    pub version_minor: u16,     // minor version number
    pub thiszone: i32,          // GMT to local correction
    pub sigfigs: u32,           // accuracy of timestamps
    pub snaplen: u32,           // max length of captured packets, in octets
    pub network: u32,           // data link type
}
