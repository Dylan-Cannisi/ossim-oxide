use core::fmt;


use crate::tiff::{constants, Tiff, TiffIfd};
use crate::tiff::constants::IfdRegisteredTag;


impl fmt::Display for Tiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut retval = "".to_string();
        retval = format!("TIFF::FILENAME: {}\n", &self.image_file);
        retval = format!("{}TIFF::NUMOFBANDS: {}\n", retval, &self.number_bands);
        retval = format!("{}TIFF::BITWIDTH: {}\n", retval, &self.bit_width);
        retval = format!("{}TIFF::BYTEORDER: {}\n", retval, &self.meta_data.byte_order);
        retval = format!("{}TIFF::MAGICNUMBER: {}\n", retval, &self.meta_data.magic_number);
        for ifd in &self.meta_data.ifds {
            retval = format!("{}{}\n", retval, ifd);
        }
        write!(f, "{}", retval)
    }
}


impl fmt::Display for TiffIfd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut retval = "".to_string();
        for entry in &self.entries {
            retval = format!("{}TIFF::{}:\n", retval,
                                constants::BASELINE.get(&entry.tag).unwrap_or(
                                    constants::GEOTIFF.get(&entry.tag).unwrap_or(
                                        &IfdRegisteredTag { readable: "UndefinedTag", field_type: constants::IfdType::Undefined}))
                                    .readable);
        }
        write!(f, "{}", retval)
    }
}