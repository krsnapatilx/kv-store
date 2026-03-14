use std::{fmt::Write, io::Read, io::Result};

#[allow(dead_code)]
pub const TOMBSTONE_MAKER: u32 = u32::MAX;

#[allow(dead_code)]
pub struct RecordHeader {
    pub key_len: u32,
    pub value_len: u32,
    pub flags: u8,
    pub checksum: u32,
}

#[allow(dead_code)]
impl RecordHeader {
    pub fn new(key_len: u32, value_len: u32, flags: u8, checksum: u32) -> Self {
        Self {
            key_len,
            value_len,
            flags,
            checksum,
        }
    }

    pub fn write_to<W: Write>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }

    pub const HEADER_SIZE: usize = 13;

    pub fn read_from<R: Read>(_reader: &mut R) -> Result<Option<Self>> {
        Ok(None)
    }
}

#[allow(dead_code)]
pub struct Record {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
    pub flags: u8,
    pub checksum: u32,
}

#[allow(dead_code)]
impl Record {
    pub fn new(key: Vec<u8>, value: Vec<u8>) -> Self {
        Self {
            key,
            value: Some(value),
            flags: 0,
            checksum: 0,
        }
    }

    pub fn tombstone(key: Vec<u8>) -> Self {
        Self {
            key,
            value: None,
            flags: 0,
            checksum: 0,
        }
    }
}

#[allow(dead_code)]
pub fn compute_checksum(key: &[u8], value: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(key);
    hasher.update(value);
    hasher.finalize()
}
