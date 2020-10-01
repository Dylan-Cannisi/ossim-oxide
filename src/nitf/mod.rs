extern crate nom;

use nom::{
    IResult,
    bytes::complete::{tag, take},
    character::complete::one_of,
};


use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;


use chrono::{DateTime, NaiveDateTime, Utc};


pub trait Loadable {
    type MyType;
    fn load(filename: String) -> std::io::Result<Self::MyType>;
}


impl Loadable for Nitf {
    type MyType = Nitf;
    fn load(filename: String) -> std::io::Result<Self::MyType> {
        let mut file_content = Vec::new();
        let mut file = File::open(&filename).expect("Unable to open NITF file");
        file.read_to_end(&mut file_content).expect("Unable to read");
        Ok(nitf_parser(&*file_content).unwrap().1)
    }
}


impl fmt::Display for Nitf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut retval = "".to_string();
        retval = format!("{}\n", &self.nitf_file_header);
        write!(f, "{}", retval)
    }
}


impl fmt::Display for NitfFileHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut retval = "".to_string();
        retval = format!("NITF::FHDR: {}\n", &self.fhdr);
        retval = format!("{}NITF::FVER: {}\n", retval, &self.fver);
        retval = format!("{}NITF::CLEVEL: {}\n", retval, &self.clevel);
        retval = format!("{}NITF::STYPE: {}\n", retval, &self.stype);
        retval = format!("{}NITF::OSTAID: {}\n", retval, &self.ostaid);
        retval = format!("{}NITF::FDT: {}\n", retval, &self.fdt);
        retval = format!("{}NITF::FTITLE: {}\n", retval, &self.ftitle);
        retval = format!("{}NITF::FSCLAS: {}\n", retval, &self.fsclas);
        retval = format!("{}NITF::FSCLSY: {}\n", retval, &self.fsclsy);
        retval = format!("{}NITF::FSCODE: {}\n", retval, &self.fscode);
        retval = format!("{}NITF::FSCTLH: {}\n", retval, &self.fsctlh);
        retval = format!("{}NITF::FSREL: {}\n", retval, &self.fsrel);
        write!(f, "{}", retval)
    }
}


fn nitf_parser(input: &[u8]) -> IResult<&[u8], Nitf> {
    let (input, header) = nitf_file_header_parser (input)?;
    Ok((input, Nitf {
        nitf_file_header: header,
        nitf_image_subheader: vec![],
        nitf_graphic_subheader: vec![],
        nitf_reserved_subheader: vec![],
        nitf_text_subheader: vec![],
        nitf_data_ext_subheader: vec![],
        nitf_reserved_ext_subheader: vec![]
    }))
}


fn nitf_file_header_parser(input: &[u8]) -> IResult<&[u8], NitfFileHeader> {
    let (input, fhdr) = tag("NITF")(input)?;
    let (input, fver) = tag("02.10")(input)?;
    let (input, clevel) = take(2 as u64)(input)?;
    let (input, stype) = tag("BF01")(input)?;
    let (input, ostaid) = take(10 as u64)(input)?;
    let (input, fdt) = take(14 as u64)(input)?;
    let (input, ftitle) = take(80 as u64)(input)?;
    let (input, fsclas) = one_of("TSCRU")(input)?;
    let (input, fsclsy) = take(2 as u64)(input)?;
    let (input, fscode) = take(11 as u64)(input)?;
    let (input, fsctlh) = take(2 as u64)(input)?;
    let (input, fsrel) = take(20 as u64)(input)?;
    Ok((input, NitfFileHeader {
        fhdr:   std::str::from_utf8(&fhdr).unwrap().parse().unwrap(),
        fver:   std::str::from_utf8(&fver).unwrap().parse().unwrap(),
        clevel: u8::from_str(std::str::from_utf8(&clevel).unwrap()).unwrap(),
        stype:  std::str::from_utf8(&stype).unwrap().parse().unwrap(),
        ostaid: std::str::from_utf8(&ostaid).unwrap().parse().unwrap(),
        fdt:    DateTime::<Utc>::from_utc(
                                    NaiveDateTime::parse_from_str(std::str::from_utf8(&fdt)
                                                        .unwrap(),
                                    "%Y%m%d%H%M%S").unwrap(), Utc
                    ),
        ftitle: std::str::from_utf8(&ftitle).unwrap().parse().unwrap(),
        fsclas,
        fsclsy: std::str::from_utf8(&fsclsy).unwrap().parse().unwrap(),
        fscode: std::str::from_utf8(&fscode).unwrap().parse().unwrap(),
        fsctlh: std::str::from_utf8(&fsctlh).unwrap().parse().unwrap(),
        fsrel:  std::str::from_utf8(&fsrel).unwrap().parse().unwrap(),

    }))
}


fn nitf_image_subheader_parser(input: &[u8]) -> IResult<&[u8], NitfImageSubheader> {
    Ok((input, NitfImageSubheader {}))
}


fn nitf_graphic_subheader_parser(input: &[u8]) -> IResult<&[u8], NitfGraphicSubheader> {
    Ok((input, NitfGraphicSubheader {}))
}


fn nitf_reserved_subheader_parser(input: &[u8]) -> IResult<&[u8], NitfReservedSubheader> {
    Ok((input, NitfReservedSubheader {}))
}


fn nitf_text_subheader_parser(input: &[u8]) -> IResult<&[u8], NitfTextSubheader> {
    Ok((input, NitfTextSubheader {}))
}


fn nitf_data_ext_subheader_parser(input: &[u8]) -> IResult<&[u8], NitfDataExtSubheader> {
    Ok((input, NitfDataExtSubheader {}))
}


fn nitf_reserved_ext_subheader_parser(input: &[u8]) -> IResult<&[u8], NitfReservedExtSubheader> {
    Ok((input, NitfReservedExtSubheader {}))
}


pub struct Nitf {
    nitf_file_header:            NitfFileHeader,
    nitf_image_subheader:        Vec<NitfImageSubheader>,
    nitf_graphic_subheader:      Vec<NitfGraphicSubheader>,
    nitf_reserved_subheader:     Vec<NitfReservedSubheader>,
    nitf_text_subheader:         Vec<NitfTextSubheader>,
    nitf_data_ext_subheader:     Vec<NitfDataExtSubheader>,
    nitf_reserved_ext_subheader: Vec<NitfReservedExtSubheader>,
}


pub struct NitfFileHeader {
    fhdr:   String,
    fver:   String,
    clevel: u8,
    stype:  String,
    ostaid: String,
    fdt:    DateTime<Utc>,
    ftitle: String,
    fsclas: char,
    fsclsy: String,
    fscode: String,
    fsctlh: String,
    fsrel:  String,
}


pub struct NitfImageSubheader {

}


pub struct NitfGraphicSubheader {

}


pub struct NitfReservedSubheader {

}


pub struct NitfTextSubheader {

}


pub struct NitfDataExtSubheader {

}


pub struct NitfReservedExtSubheader {

}