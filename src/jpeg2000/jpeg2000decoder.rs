use std::io::Error;


use crate::image::{IsImageBuffer, IsImageDecoder, IsImageData};


impl IsImageBuffer for jp2k::ImageBuffer {
    fn get_buffer(self: &Self) -> Vec<u8> {
        self.buffer.clone()
    }

    fn get_width(self: &Self) -> u64 {
        self.width as u64
    }

    fn get_height(self: &Self) -> u64 {
        self.height as u64
    }

    fn get_num_bands(self: &Self) -> u16 {
        self.num_bands as u16
    }

    fn get_bit_depth(self: &Self) -> u16 {
        8 as u16
    }
}


impl IsImageDecoder for jp2k::ImageBuffer {
    type MyType = jp2k::ImageBuffer;

    fn decode<T: IsImageData>(image: &T) -> Result<jp2k::ImageBuffer, Error> {
        let bytes = image.get_image_data();

        let codec = jp2k::Codec::jp2();
        let stream = jp2k::Stream::from_bytes(&*bytes).unwrap();

        let retval = jp2k::ImageBuffer::build(
            codec,
            stream,
            jp2k::DecodeParams::default().with_reduce_factor(1),
        ).unwrap();

        Ok(retval)
    }
}