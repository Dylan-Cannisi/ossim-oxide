mod constants;
mod display;
mod parser;

use std::error::Error;
use std::fs::File;
use std::fmt;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use core::convert::From;


use image::{
    DynamicImage,
    ImageFormat,
};

use crate::image::Loadable;
use crate::flatten_vec_u16;


pub struct Tiff {
    image_file: String,
    number_bands: usize,
    bit_width: usize,
    meta_data: TiffMetadata,
    pixel_data: Vec<u8>,
}


pub struct TiffMetadata {
    byte_order: String,  // In the field it is either II for little endian or MM for big endian
    magic_number: u16,   // Must be 42
    ifds: Vec<TiffIfd>
}


pub struct TiffIfd {
    num_entries: u16,
    entries: Vec<TiffIfdEntry>
}


pub struct TiffIfdEntry {
    tag: u16,
    field_type: u16,
    num_values: u32,
    offset: u32,
    value: Vec<u8>
}


impl Loadable for Tiff {
    type MyType = Tiff;
    fn load(filename: String) -> std::io::Result<Self::MyType> {
        let mut f = File::open(&filename).expect("Could not open Tiff file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let image_data = match image::load_from_memory_with_format(&*buffer, ImageFormat::Tiff) {
            Ok(image) => image,
            _ => panic!("Unable to open TIFF image.")
        };

        let (_, metadata) = parser::tiff_parser(&buffer)
            .expect("Could not parse metadata");

        return match image_data {
            DynamicImage::ImageLuma8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    meta_data: metadata,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageLumaA8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    meta_data: metadata,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageRgb8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    meta_data: metadata,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageRgba8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    meta_data: metadata,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageBgr8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    meta_data: metadata,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageBgra8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    meta_data: metadata,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageLuma16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    meta_data: metadata,
                    pixel_data: flatten_vec_u16(image.into_vec())
                                    .expect("Unable to flatten image data.")
                })
            },
            DynamicImage::ImageLumaA16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    meta_data: metadata,
                    pixel_data: flatten_vec_u16(image.into_vec())
                                    .expect("Unable to flatten image data.")
                })
            },
            DynamicImage::ImageRgb16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    meta_data: metadata,
                    pixel_data: flatten_vec_u16(image.into_vec())
                                    .expect("Unable to flatten image data.")
                })
            },
            DynamicImage::ImageRgba16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    meta_data: metadata,
                    pixel_data: flatten_vec_u16(image.into_vec())
                                    .expect("Unable to flatten image data.")
                })
            },
        };
    }
}





