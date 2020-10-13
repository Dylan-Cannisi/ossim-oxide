extern crate nom;
use nom::{
    bytes::complete::{tag, take},
    character::complete::one_of,
    IResult,
    lib::std::collections::HashMap,
};

use crate::tiff::{TiffIfd, TiffIfdEntry, TiffMetadata,};
use self::nom::multi::many1_count;
use self::nom::sequence::pair;


pub(crate) fn tiff_parser(input: &[u8]) -> IResult<&[u8], TiffMetadata> {
    let (input_t, byte_order) = take(2 as u64) (input)?;
    let (input_t, magic_number) = take(2 as u64) (input_t)?;

    let (input_t, ifd_offset) = take(4 as u64) (input_t)?;
    let offset = match std::str::from_utf8(&byte_order).unwrap() {
        "II" =>
            u32::from_le_bytes(*slice_as_array!(ifd_offset, [u8; 4]).expect("bad hash length")),
        "MM" =>
            u32::from_be_bytes(*slice_as_array!(ifd_offset, [u8; 4]).expect("bad hash length")),
        _ => 0 as u32
    };
    let (input_t, _) = take(offset) (input)?;

    let (input_t, ifd) = tiff_ifd_parser(input_t, std::str::from_utf8(&byte_order).unwrap())?;
    let mut ifds = Vec::new();
    ifds.push(ifd);

    Ok((input, TiffMetadata {
        byte_order: std::str::from_utf8(&byte_order).unwrap().parse().unwrap(),
        magic_number: match std::str::from_utf8(&byte_order).unwrap() {
            "II" =>
                u16::from_le_bytes(*slice_as_array!(magic_number, [u8; 2]).expect("bad hash length")),
            "MM" =>
                u16::from_be_bytes(*slice_as_array!(magic_number, [u8; 2]).expect("bad hash length")),
            _ => 0 as u16
        },
        ifds,
    }))
}


fn tiff_ifd_parser<'a>(input: &'a [u8], byte_order: &'a str) -> IResult<&'a [u8], TiffIfd> {
    let (input, num_entries) = take(2 as u64) (input)?;
    let num = match byte_order {
        "II" =>
            u16::from_le_bytes(*slice_as_array!(num_entries, [u8; 2]).expect("bad hash length")),
        "MM" =>
            u16::from_be_bytes(*slice_as_array!(num_entries, [u8; 2]).expect("bad hash length")),
        _ => 0 as u16
    };

    let mut entries = Vec::new();
    for i in 1..num {
        let (input, _) = take(((i-1)*12) as u64) (input)?;
        let (input, entry) = tiff_ifd_entry_parser(input,byte_order)?;
        entries.push(entry);
    }

    Ok((input, TiffIfd {
        num_entries: num,
        entries
    }))
}


fn tiff_ifd_entry_parser<'a>(input: &'a [u8], byte_order: &'a str) -> IResult<&'a [u8], TiffIfdEntry> {
    let (input, tag) = take(2 as u64) (input)?;
    let (input, field_type) = take(2 as u64) (input)?;
    let (input, num_values) = take(4 as u64) (input)?;
    let (input, offset) = take(4 as u64) (input)?;

    Ok((input, TiffIfdEntry {
        tag: match byte_order {
            "II" =>
                u16::from_le_bytes(*slice_as_array!(tag, [u8; 2]).expect("bad hash length")),
            "MM" => u16::from_be_bytes(*slice_as_array!(tag, [u8; 2]).expect("bad hash length")),
            _ => 0 as u16,
        },
        field_type: match byte_order {
            "II" => u16::from_le_bytes( * slice_as_array ! (field_type, [u8; 2]).expect("bad hash length")),
            "MM" => u16::from_be_bytes( * slice_as_array ! (field_type, [u8; 2]).expect("bad hash length")),
            _ => 0 as u16,
        },
        num_values: match byte_order {
            "II" => u32::from_le_bytes( * slice_as_array ! (num_values, [u8; 4]).expect("bad hash length")),
            "MM" => u32::from_be_bytes( * slice_as_array ! (num_values, [u8; 4]).expect("bad hash length")),
            _ => 0 as u32,
        },
        offset: match byte_order {
            "II" => u32::from_le_bytes( * slice_as_array ! (offset, [u8; 4]).expect("bad hash length")),
            "MM" => u32::from_be_bytes( * slice_as_array ! (offset, [u8; 4]).expect("bad hash length")),
            _ => 0 as u32,
        },
        value: vec![]
    }))
}