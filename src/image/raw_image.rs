use crate::image::{IsImageData, Loadable};

use std::fs;
use std::fs::File;
use std::io::Read;


pub struct RawImageData {
    image_file: String,
    image_data: Vec<u8>,
}

impl Loadable for RawImageData {
    type MyType = RawImageData;

    fn load(filename: String) -> std::io::Result<RawImageData> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        Ok(RawImageData {
            image_file: filename,
            image_data: buffer
        })
    }
}

impl IsImageData for RawImageData {
    fn get_image_data(self: &Self) -> Vec<u8> {
        self.image_data.clone()
    }
}