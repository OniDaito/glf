//!    ___  __    ____ 
//!   / __)(  )  (  __)
//!  ( (_ \/ (_/\ ) _) 
//!   \___/\____/(__) 
//!   
//! # Overview
//! The main file that represents our GLF

use crate::ciheader::parse_header;
use crate::imagerec::parse_image_record;
use crate::statusrec::parse_status_record;
use crate::{ImageRecord, StatusRecord};
use image::{GrayImage, ImageBuffer, Luma};
use zune_inflate::DeflateDecoder;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Read, Seek};
use std::vec;

#[derive(Clone)]
pub struct GLF {
    /// The path to the GLF file.
    pub filepath: PathBuf,
    /// A vector of the ImageRecords in time order.
    pub images: Vec<ImageRecord>,
    /// A vector of StatusRecords in time order.
    pub statuses: Vec<StatusRecord>,
    /// The raw data as a vector of bytes.
    pub dat: Vec<u8>,
}

/// A small struct that holds the Image but also it's frame number.
pub struct NidxImg {
    /// Frame number of this image.
    pub idx: u32,
    /// The image itself.
    pub img: ImageBuffer<Luma<u8>, Vec<u8>>
}

/// GLF files are actually zip files (sort of), so we first perform an unzip
/// with this function.
/// 
/// * `reader` - object that implements Read and Seek
fn read_zip_dat(reader: impl Read + Seek) -> Option<Vec<u8>> {
    match zip::ZipArchive::new(reader) {
        Ok(mut zip) => {
            for i in 0..zip.len() {
                match zip.by_index(i) {
                    Ok(mut file) => {
                        // Should be three files inside the GLF - .cfg, .dat and .xml.
                        if file.name().contains("dat") {
                            let mut buffer: Vec<u8> = vec![];
                           
                            match  file.read_to_end(&mut buffer) {
                                Ok(_) => return Some(buffer),
                                Err(_) => return None,
                            }
                        }
                    },
                    Err(_) => return None,
                }
            }
        }
        Err(..) => {
            return None
        }
    }
    None
}
 
/// The main parse function that goes through the entire dat_buffer,
/// and returns the records for use later.
/// 
/// * `dat_buffer` - a vector of byte.
fn parse_dat(dat_buffer: &Vec<u8>) -> (Vec<ImageRecord>, Vec<StatusRecord>) {
    let mut file_offset: i64 = 0;
    let mut image_records: Vec<ImageRecord> = vec![];
    let mut status_records: Vec<StatusRecord> = vec![];

    while file_offset < dat_buffer.len() as i64 - 2 {
        let header = parse_header(dat_buffer, &mut file_offset);

        if header.header_type == 0 {
            // image record
            let image_rec = parse_image_record(&header, dat_buffer, &mut file_offset);
            image_records.push(image_rec);
            
        } else if header.header_type == 1 {
            // V4 Protocol
            assert!(false);
        } else if header.header_type == 2 {
            // analog video
            assert!(false);
        } else if header.header_type == 3 {
            // Gemini Status
            let status_rec = parse_status_record(&header, dat_buffer, &mut file_offset);
            status_records.push(status_rec);
        } else if header.header_type == 98 {
            // Raw Serial
            assert!(false);
        } else if header.header_type == 99 {
            // Generic
            assert!(false);
        } else {
            // Incorrect
            assert!(false);
        }
    }

    (image_records, status_records)
}

impl GLF {
    /// Create a new GLF object from the glf file on disk.
    /// 
    /// * `path` - the Path to the GLF file
    pub fn new(path: &Path) -> Result<GLF, &'static str>{
        match File::open(path) {
            Ok(f) => {
                match read_zip_dat(f) {
                    Some(dat_buffer) => {
                        // Now create the GLF - just parse images more or less and return.
                        let (images, statuses) = parse_dat(&dat_buffer);

                        let glf = GLF {
                            filepath: path.to_path_buf(),
                            images: images,
                            statuses: statuses,
                            dat: dat_buffer,
                        };
                        // We now have a data buffer for the .dat file inside the glf zip.
                        return Ok(glf);
                    },
                    None => { return Err("Error parsing GLF."); }
                }
            },
            Err(_) => return Err("Failed to open GLF File")
        }
    }

    pub fn len(&self) -> usize {
        //! Return the number of images in this GLF
        self.images.len()
    }

    /// Extract an image from the GLF file.
    /// 
    /// * `idx` - the index of the image we want.
    pub fn extract_image(&self, idx: usize) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, &'static str> {
        // Extract the image itself, given the idx of the record.
        // Return it as a image buffer.
        // We need to read the area of the dat file and potentially unzip it.
        let img_rec = &self.images[idx];
        let ptr = img_rec.data_ptr;
        let dat_size = img_rec.data_size;
        if ptr + dat_size < self.dat.len() as u32 {
            let raw_img_data = self.dat.get(ptr as usize..((ptr + dat_size) as usize)).unwrap();
            let width = img_rec.image_width;
            let height = img_rec.image_height;

            if img_rec.compression_type == 0 {
                let mut decoder = DeflateDecoder::new(&raw_img_data);
                let decompressed_data = decoder.decode_zlib().unwrap();
                let img: ImageBuffer<Luma<u8>, Vec<u8>> = GrayImage::from_vec(width, height, decompressed_data).unwrap();
                return Ok(img);
        
            } else if img_rec.compression_type == 2 {
                return Err("H264 decompression not yet implemented.");
            }

            let img: ImageBuffer<Luma<u8>, Vec<u8>> = GrayImage::from_vec(width, height, raw_img_data.to_vec()).unwrap();
            return Ok(img);
        } 
            
        return Err("ptr exceeds image data length");
    }

    /// Extract the image itself, given the idx of the record and a sonar_id. 
    /// Return it as a image buffer.
    /// We need to read the area of the dat file and potentially unzip it.
    /// We return the idx of the 'next' record matching the sonar id.
    ///
    /// * `idx` - the index of the image we want.
    /// * `sonar_id` - the id of the sonar we want to extract for, in the case of mulitplexed GLFs.
    pub fn extract_image_next_sonarid(&self, idx: usize, sonar_id: u16) -> Option<NidxImg> {
       
        let mut tidx = idx;
        let mut img_rec = &self.images[tidx];
        
        while img_rec.header.device_id != sonar_id {
            tidx += 1;
            
            if tidx >= self.images.len() {
                return None          
            }
            
            img_rec = &self.images[tidx];
        }
        
        let mut nidx = tidx + 1;
        
        if nidx >= self.images.len() {
            return None;
        }
        
        let mut next_rec = &self.images[nidx];
        
        while next_rec.header.device_id != sonar_id {
            nidx += 1;
            
            if nidx >= self.images.len() {
                return None;
            }

            next_rec = &self.images[nidx];
        }
       
        match self.extract_image(tidx) {
            Ok(img) => { return Some(NidxImg{idx: nidx as u32, img: img}); }
            Err(_) => {None},            
        }

    }
}

impl std::fmt::Display for GLF {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filepath.to_str().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glf() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("pytritech_testdata/test_tritech.glf");
        let glf = GLF::new(Path::new(&d)).unwrap();
        println!("GLF Image 0: {}", glf.images[0].header.time);
        let img = glf.extract_image(1).unwrap();
        img.save("test.png").unwrap();
    }
}

