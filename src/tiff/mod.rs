use std::fs::File;
use std::io::Read;


use image::{
    DynamicImage,
    ImageFormat,
};


use crate::image::Loadable;


pub struct Tiff {
    image_file: String,
    number_bands: usize,
    bit_width: usize,
    pixel_data: Vec<u8>,
}


impl<T> Loadable for Tiff<T> {
    type MyType = Tiff<T>;
    fn load(filename: String) -> std::io::Result<Self::MyType> {
        let mut f = File::open(&filename).expect("Could not open Tiff file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let image_data = match image::load_from_memory_with_format(&*buffer, ImageFormat::Tiff) {
            Ok(image) => image,
            _ => panic!("Unable to open TIFF image.")
        };

        return match image_data {
            DynamicImage::ImageLuma8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageLumaA8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageRgb8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageRgba8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageBgr8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageBgra8(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 8,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageLuma16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageLumaA16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageRgb16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    pixel_data: image.into_vec()
                })
            },
            DynamicImage::ImageRgba16(image) => {
                Ok(Tiff {
                    image_file: filename,
                    number_bands: 1,
                    bit_width: 16,
                    pixel_data: image.into_vec()
                })
            },
        };
    }
}