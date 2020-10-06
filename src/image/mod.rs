mod raw_image;

pub trait IsImageBuffer {
    fn get_buffer(self: &Self) -> Vec<u8>;
    fn get_width(self: &Self) -> u64;
    fn get_height(self: &Self) -> u64;
    fn get_num_bands(self: &Self) -> u16;
    fn get_bit_depth(self: &Self) -> u16;
}


pub trait IsImageData {
    fn get_image_data(self: &Self) -> Vec<u8>;
}


pub trait IsImageDecoder {
    type MyType;
    fn decode<T: IsImageData>(image: &T) -> std::io::Result<Self::MyType>;
}

pub trait Loadable {
    type MyType;
    fn load(filename: String) -> std::io::Result<Self::MyType>;
}