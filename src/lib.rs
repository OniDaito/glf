//! # Overview
//! This crate provides a native rust implementation of the code required to read
//! the [Tritech GLF](https://www.tritech.co.uk/products/gemini-720ik) files.
//! 
//! This crate makes extensive use of the Rust [Image](https://crates.io/crates/image) crate,
//! using the ```ImageBuffer<Luma<u8>, Vec<u8>>``` as it's main type for holding the image data.
//! 
//! # Example usage
//!
//! ```
//! use std::path::Path;
//! use glf::GLF;
//!
//! let glf = GLF::new(Path::new("./pytritech_testdata/test_tritech.glf")).unwrap();
//! println!("GLF Image 0: {}", glf.images[0].header.time);
//! let img = glf.extract_image(1).unwrap();
//! img.save("test.png").unwrap();
//! ```

mod ciheader;
mod glf;
mod imagerec;
mod epochgem;
mod statusrec;

pub use crate::imagerec::ImageRecord;
pub use crate::statusrec::StatusRecord;
pub use crate::ciheader::CIHeader;
pub use crate::glf::GLF;
pub use crate::epochgem::epoch_gem;
