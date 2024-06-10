//!    ___  __    ____ 
//!   / __)(  )  (  __)
//!  ( (_ \/ (_/\ ) _) 
//!   \___/\____/(__) 
//!   
//! # Overview
//! The CIHeader struct
//! 
//! <https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#examine-the-date-and-time>
 
use crate::epoch_gem;
use chrono::{DateTime, Utc};
use core::time::Duration;
use byteorder::{ByteOrder, LittleEndian};
use std::fmt;

#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
#[non_exhaustive]
pub struct CIHeader {
    /// The size of the header in bytes.
    pub header_size: u8,
    /// The payload length in bytes.
    pub payload_length: u32,
    /// The time in UTC.
    pub time: DateTime<Utc>,
    /// The type of the header.
    pub header_type: u8,
    /// The device ID (the sonar id).
    pub device_id: u16,
    /// Node ID.
    pub node_id: u16,
}

impl CIHeader {

    /// Create a new CIHeader with the default parameters
    pub fn new() -> CIHeader {
        CIHeader {
            header_size: 21,
            payload_length: 0,
            time: Utc::now(),
            header_type: 0,
            device_id: 0,
            node_id: 0
        }
    }

    /// Return the size of this header in bytes.
    pub fn len(self) -> u32 {
        self.header_size as u32
    }
}

impl fmt::Display for CIHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {})", self.payload_length, self.time, self.header_type, self.device_id, self.node_id)
    }
}


/// Extract the header from this part of the dat_buffer. Change the file_offset
/// as a result.
/// 
/// * `dat_buffer` - a vector of byte.
/// * `file_offset` - current offset in the buffer. 
pub fn parse_header(dat_buffer: &Vec<u8>, file_offset: &mut i64) -> CIHeader{
    // Parse a header, moving the file_offset along.
    let fp: usize = *file_offset as usize;
    let mut header = CIHeader::new();
    assert!(dat_buffer[0] as char == '*');
    // missing byte here, for version, is ignored for now
    header.payload_length = LittleEndian::read_u32(&dat_buffer[(fp + 2)..(fp + 6)]) - (header.header_size as u32);
    let tts = LittleEndian::read_f64(&dat_buffer[(fp + 6)..(fp + 14)]);
    let tmillis = (tts as f64 * 1000.0).round() as u64;
    let dur : Duration = Duration::from_millis(tmillis);
    let epoch: chrono::prelude::DateTime<chrono::prelude::Utc> = epoch_gem();
    header.time = epoch + dur;

    header.header_type = dat_buffer[fp + 14];
    header.device_id = LittleEndian::read_u16(&dat_buffer[(fp + 15)..(fp + 17)]);
    header.node_id = LittleEndian::read_u16(&dat_buffer[(fp + 17)..(fp + 19)]);

    *file_offset = *file_offset + (header.header_size as i64);

    header
}