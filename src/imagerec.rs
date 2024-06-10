//!    ___  __    ____ 
//!   / __)(  )  (  __)
//!  ( (_ \/ (_/\ ) _) 
//!   \___/\____/(__) 
//!   
//! # ImageRecord
//! An struct that holds the image data, with it's own header and a CIHeader.

use chrono::{DateTime, Utc};
use core::time::Duration;
use std::vec;
use byteorder::{ByteOrder, LittleEndian};
use crate::{CIHeader, epoch_gem};


/// The image record holds all the information on a single frame / image
/// from the sonar, including the starting position in the byte array
/// for this image.
#[derive(Clone)]
pub struct ImageRecord {
    /// The CIHeader
    pub header: CIHeader,
    /// Version number
    pub version: u16,
    /// Image version number.
    pub image_version: u16,
    /// The starting range in metres.
    pub range_start: u32,
    /// End of the range in metres.
    pub range_end: u32,
    /// Range compression.
    pub range_compression: u16,
    /// Starting bearing in degrees.
    pub bearing_start: u32,
    /// Ending bearing in degrees.
    pub bearing_end: u32,
    /// Compression type.
    pub compression_type: u16,
    /// Pointer into the data buffer.
    pub data_ptr: u32,
    /// The number of bytes to read.
    pub data_size: u32,
    /// The bearing table for this image.
    pub bearing_table: Vec<f64>,
    /// Any state flags.
    pub state_flags: u32,
    /// Modulation frequency.
    pub modulation_frequency: u32,
    /// Beam forming
    pub beam_form_app: f32,
    /// The transmission time in UTC
    pub db_tx_time: DateTime<Utc>,
    /// Any ping flags.
    pub ping_flags: u16,
    /// sos at xd.
    pub sos_at_xd: f32,
    /// Percentage gain.
    pub percent_gain: u16,
    /// CHIRP mode on?
    pub chirp: u8,
    /// The type of the sonar.
    pub sonar_type: u8,
    /// The platform id.
    pub platform: u8,
    /// Size of the record.
    pub record_size: u32, 
    /// The width of the image in pixels.
    pub image_width: u32,
    /// The height of the image in pixels.
    pub image_height: u32,
}


/// Extract the image itself, given the idx of the record and a sonar_id. 
///
/// * `header` - the CI Header for this record.
/// * `dat_buffer` - the bytes buffer we are reading from.
/// * `file_offset` - the offset in the dat_buffer.
pub fn parse_image_record(header: &CIHeader, dat_buffer: &Vec<u8>, file_offset: &mut i64) -> ImageRecord {
    // Parse a record - an image one for now.
    let mut fp: usize = *file_offset as usize;

    let rtype = LittleEndian::read_u16(&dat_buffer[fp..(fp + 2)]);
    assert!(rtype == 1);
    let version = LittleEndian::read_u16(&dat_buffer[(fp + 2)..(fp + 4)]);
    assert!(version == 0xEFEF);
    fp = fp + 4; // Advance the FP.

    let image_version = LittleEndian::read_u16(&dat_buffer[fp..(fp + 2)]);
    let range_start = LittleEndian::read_u32(&dat_buffer[(fp + 2)..(fp + 6)]);
    let range_end = LittleEndian::read_u32(&dat_buffer[(fp + 6)..(fp + 10)]);
    let range_compression = LittleEndian::read_u16(&dat_buffer[(fp + 10)..(fp + 12)]);
    let bearing_start = LittleEndian::read_u32(&dat_buffer[(fp + 12)..(fp + 16)]);
    let bearing_end = LittleEndian::read_u32(&dat_buffer[(fp + 16)..(fp + 20)]);

    fp = fp + 20; // Advance fp again.

    let mut compression_type: u16 = 1;
    
    if image_version == 3 {
        compression_type = LittleEndian::read_u16(&dat_buffer[fp..(fp + 2)]);
        fp = fp + 2;
    }

    let dat_size = LittleEndian::read_u32(&dat_buffer[fp..(fp + 4)]);
    let dat_ptr = fp + 4;
    fp = fp + 4 + dat_size as usize;

    let bsize = bearing_end - bearing_start;
    let mut btable: Vec<f64> = vec![];

    for i in 0..bsize {
        let bearing = LittleEndian::read_f64(&dat_buffer[fp + (i * 8) as usize..fp + ((i + 1) * 8) as usize]);
        btable.push(bearing);
    }

    fp = fp + (bsize * 8) as usize;

    let state_flags = LittleEndian::read_u32(&dat_buffer[fp..(fp + 4)]);
    let modulation_frequency = LittleEndian::read_u32(&dat_buffer[(fp + 4)..(fp + 8)]);

    fp = fp + 8;

    let beam_form = LittleEndian::read_f32(&dat_buffer[fp..(fp + 4)]);

    // Get the timing
    let tts = LittleEndian::read_f64(&dat_buffer[(fp + 4)..(fp + 12)]);
    let tmillis = (tts as f64 * 1000.0).round() as u64;
    let dur : Duration = Duration::from_millis(tmillis);
    let epoch: chrono::prelude::DateTime<chrono::prelude::Utc> = epoch_gem();
    let db_tx_time = epoch + dur;

    let ping_flags = LittleEndian::read_u16(&dat_buffer[(fp + 12)..(fp + 14)]);
    let sos_at_xd = LittleEndian::read_f32(&dat_buffer[(fp + 14)..(fp + 18)]);
    let percent_gain = LittleEndian::read_u16(&dat_buffer[(fp + 18)..(fp + 20)]);
    let chirp = dat_buffer[fp + 20];
    let sonar_type = dat_buffer[fp + 21];
    let platform = dat_buffer[fp + 22];

    // Note the extra byte pad!
    let end_tag = LittleEndian::read_u16(&dat_buffer[(fp + 24)..(fp + 26)]);
    assert!(end_tag == 0xDEDE);

    fp = fp + 26;
    let record_size = fp - *file_offset as usize;
    let image_width = bearing_end - bearing_start;
    let image_height = range_end - range_start;

    // Deal with potential compression.
    if image_version != 3 {
        let exp_size = (bearing_end - bearing_start) * (range_end - range_start);
        
        if exp_size != dat_size {
            compression_type = 0;
        }
    }

    // Setup the beginning of our ImageRecord
    let img_rec : ImageRecord = ImageRecord {
        header: *header,
        version: version,
        image_version: image_version,
        range_start: range_start,
        range_end: range_end,
        range_compression: range_compression,
        bearing_start: bearing_start,
        bearing_end: bearing_end,
        compression_type: compression_type,
        data_ptr: dat_ptr as u32,
        data_size: dat_size,
        bearing_table: btable,
        state_flags: state_flags,
        modulation_frequency: modulation_frequency,
        beam_form_app: beam_form,
        db_tx_time: db_tx_time,
        ping_flags: ping_flags,
        sos_at_xd: sos_at_xd,
        percent_gain: percent_gain,
        chirp: chirp,
        sonar_type: sonar_type,
        platform: platform,
        record_size: record_size as u32,
        image_width: image_width,
        image_height: image_height,
    };

    *file_offset = *file_offset + (record_size as i64);
    img_rec
}
