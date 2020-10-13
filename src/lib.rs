pub mod image;
pub mod jpeg2000;
pub mod nitf;
pub mod tiff;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::io::Result;


#[macro_use]
extern crate slice_as_array;


extern crate byteorder;
use byteorder::{LittleEndian, WriteBytesExt};

fn flatten_vec_u16(vec: Vec<u16>) -> Result<Vec<u8>> {
    let mut retval = Vec::new();

    for element in vec {
        let mut bytes = [0u8; 2];
        bytes.as_mut()
            .write_u16::<LittleEndian>(element)
            .expect("Unable to write");
        for byte in &bytes {
            retval.push(*byte);
        }
    }

    Ok(retval)
}
