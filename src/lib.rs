use std::{array::TryFromSliceError, io::Write};

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

#[derive(Debug)]
pub enum SniffError {
    NoSupportedDevices,
    Open,
    Rusb(rusb::Error),
    Parse,
    MissingUsbDevice,
    TryFromSlice,
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

pub struct PcapHeader {
    pub magic_number: u32,      // magic number
    pub version_major: u16,     // major version number
    pub version_minor: u16,     // minor version number
    pub thiszone: i32,          // GMT to local correction
    pub sigfigs: u32,           // accuracy of timestamps
    pub snaplen: u32,           // max length of captured packets, in octets
    pub network: u32,           // data link type
}

pub struct PcapRecordHeader {
    pub ts_sec: u32,            // timestamp seconds
    pub ts_usec: u32,           // timestamp microseconds
    pub incl_len: u32,          // number of octets of packet saved in file
    pub orig_len: u32,          // actual length of packet
}

impl PcapHeader {
    pub fn as_array(&self) -> [u8; 24] {
        let mut arr = [0; 24];
        arr[0..4].copy_from_slice(&self.magic_number.to_le_bytes());
        arr[4..6].copy_from_slice(&self.version_major.to_le_bytes());
        arr[6..8].copy_from_slice(&self.version_minor.to_le_bytes());
        arr[8..12].copy_from_slice(&self.thiszone.to_le_bytes());
        arr[12..16].copy_from_slice(&self.sigfigs.to_le_bytes());
        arr[16..20].copy_from_slice(&self.snaplen.to_le_bytes());
        arr[20..24].copy_from_slice(&self.network.to_le_bytes());
        arr
    }
}

impl PcapRecordHeader {
    pub fn as_array(&self) -> [u8; 16] {
        let mut arr = [0; 16];
        arr[0..4].copy_from_slice(&self.ts_sec.to_le_bytes());
        arr[4..8].copy_from_slice(&self.ts_usec.to_le_bytes());
        arr[8..12].copy_from_slice(&self.incl_len.to_le_bytes());
        arr[12..16].copy_from_slice(&self.orig_len.to_le_bytes());
        arr
    }
}

pub struct Pcap {
    file: std::fs::File,
}

impl Pcap {
    pub fn new(filename: &str) -> std::io::Result<Self> {
        let file = std::fs::File::create(filename)?;
        Ok(Pcap { file })
    }

    pub fn write_header(&mut self) -> std::io::Result<()> {
        let header = PcapHeader {
            magic_number: 0xA1B2C3D4,
            version_major: 2,
            version_minor: 4,
            thiszone: 0,
            sigfigs: 0,
            snaplen: 128,
            network: 195,
        };
        let header_bytes = header.as_array();
        self.file.write_all(&header_bytes)?;
        Ok(())
    }

    pub fn write_record(&mut self, record: &[u8]) -> std::io::Result<()> {
        let record_header = PcapRecordHeader {
            ts_sec: 0,
            ts_usec: 0,
            incl_len: record.len() as u32,
            orig_len: record.len() as u32,
        };
        let record_header_bytes = record_header.as_array();
        self.file.write_all(&record_header_bytes)?;
        self.file.write_all(record)?;
        Ok(())
    }
}