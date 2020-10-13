use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;


extern crate nom;
use nom::{
    bytes::complete::{tag, take},
    character::complete::one_of,
    IResult,
    lib::std::collections::HashMap,
};

use chrono::{Date, DateTime, NaiveDateTime, Utc, NaiveDate};


use crate::image::Loadable;


fn nitf_tre_parser(input: &[u8]) -> IResult<&[u8], (usize, String, Vec<u8>)> {
    let (input, key_t) = take(6 as u64)(input)?;
    let key = std::str::from_utf8(&key_t).unwrap().trim().parse().unwrap();
    let (input, size_t) = take(5 as u64)(input)?;
    let size = usize::from_str(std::str::from_utf8(&size_t).unwrap()).unwrap();
    let (input, value) = take(size)(input)?;
    Ok((input, (size+11,key,value.to_vec())))
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
    fhdr:    String,
    fver:    String,
    clevel:  u8,
    stype:   String,
    ostaid:  String,
    fdt:     DateTime<Utc>,
    ftitle:  String,
    fsclas:  char,
    fsclsy:  String,
    fscode:  String,
    fsctlh:  String,
    fsrel:   String,
    fsdctp:  String,
    fsdcdt:  Option<Date<Utc>>,
    fsdcxm:  String,
    fsdg:    char,
    fsdgdt:  Option<Date<Utc>>,
    fsctlx:  String,
    fscatp:  char,
    fscaut:  String,
    fscrsn:  char,
    fssrdt:  Option<Date<Utc>>,
    fsctln:  String,
    fscop:   u32,
    fscpys:  u32,
    encryp:  u8,
    fbkgc_r: u8,
    fbkgc_g: u8,
    fbkgc_b: u8,
    oname:   String,
    ophone:  String,
    fl:      usize,
    hl:      usize,
    numi:    usize,
    lish:    Vec<usize>,
    li:      Vec<usize>,
    nums:    usize,
    lssh:    Vec<usize>,
    ls:      Vec<usize>,
    numx:    usize,
    numt:    usize,
    ltsh:    Vec<usize>,
    lt:      Vec<usize>,
    numdes:  usize,
    ldsh:    Vec<usize>,
    ld:      Vec<usize>,
    numres:  usize,
    lresh:   Vec<usize>,
    lre:     Vec<usize>,
    udhdl:   usize,
    udhofl:  Option<usize>,
    udhd:    Option<HashMap<String,Vec<u8>>>,
    xhdl:    usize,
    xhdlofl: Option<usize>,
    xhd:     Option<HashMap<String,Vec<u8>>>,
}


pub struct NitfImageSubheader {
    im:     String,
    iid1:   String,
    idatim: DateTime<Utc>,
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
        retval = match &self.fsdcdt {
            Some(t) => format!("{}NITF::FSDCDT: {}\n", retval, &t),
            None => retval,
        };
        retval = format!("{}NITF::FSDCXM: {}\n", retval, &self.fsdcxm);
        retval = format!("{}NITF::FSDG: {}\n", retval, &self.fsdg);
        retval = match &self.fsdgdt {
            Some(t) => format!("{}NITF::FSDGDT: {}\n", retval, &t),
            None => retval,
        };
        retval = format!("{}NITF::FSCTLX: {}\n", retval, &self.fsctlx);
        retval = format!("{}NITF::FSCATP: {}\n", retval, &self.fscatp);
        retval = format!("{}NITF::FSCAUT: {}\n", retval, &self.fscaut);
        retval = format!("{}NITF::FSCRSN: {}\n", retval, &self.fscrsn);
        retval = match &self.fssrdt {
            Some(t) => format!("{}NITF::FSSRDT: {}\n", retval, &t),
            None => retval,
        };
        retval = format!("{}NITF::FSCTLN: {}\n", retval, &self.fsctln);
        retval = format!("{}NITF::FSCOP: {}\n", retval, &self.fscop);
        retval = format!("{}NITF::FSCPYS: {}\n", retval, &self.fscpys);
        retval = format!("{}NITF::ENCRYP: {}\n", retval, &self.encryp);
        retval = format!("{}NITF::FBKGC: 0x{:02X}{:02X}{:02X}\n", retval, &self.fbkgc_r, &self.fbkgc_g, &self.fbkgc_b);
        retval = format!("{}NITF::ONAME: {}\n", retval, &self.oname);
        retval = format!("{}NITF::OPHONE: {}\n", retval, &self.ophone);
        retval = format!("{}NITF::FL: {}\n", retval, &self.fl);
        retval = format!("{}NITF::HL: {}\n", retval, &self.hl);
        retval = format!("{}NITF::NUMI: {}\n", retval, &self.numi);
        for i in 0..self.numi {
            retval = format!("{}NITF::LISH{:03}: {}\n", retval, i, &self.lish[i]);
            retval = format!("{}NITF::LI{:03}: {}\n", retval, i, &self.li[i]);
        }
        retval = format!("{}NITF::NUMS: {}\n", retval, &self.nums);
        for i in 0..self.nums {
            retval = format!("{}NITF::LSSH{:03}: {}\n", retval, i, &self.lssh[i]);
            retval = format!("{}NITF::LS{:03}: {}\n", retval, i, &self.ls[i]);
        }
        retval = format!("{}NITF::NUMX: {}\n", retval, &self.numx);
        retval = format!("{}NITF::NUMT: {}\n", retval, &self.numt);
        for i in 0..self.numt {
            retval = format!("{}NITF::LTSH{:03}: {}\n", retval, i, &self.ltsh[i]);
            retval = format!("{}NITF::LT{:03}: {}\n", retval, i, &self.lt[i]);
        }
        retval = format!("{}NITF::NUMDES: {}\n", retval, &self.numdes);
        for i in 0..self.numdes {
            retval = format!("{}NITF::LDSH{:03}: {}\n", retval, i, &self.ldsh[i]);
            retval = format!("{}NITF::LD{:03}: {}\n", retval, i, &self.ld[i]);
        }
        retval = format!("{}NITF::NUMRES: {}\n", retval, &self.numres);
        for i in 0..self.numres {
            retval = format!("{}NITF::LRESH{:03}: {}\n", retval, i, &self.lresh[i]);
            retval = format!("{}NITF::LRE{:03}: {}\n", retval, i, &self.lre[i]);
        }
        retval = format!("{}NITF::UDHDL: {}\n", retval, &self.udhdl);
        retval = match &self.udhofl {
            Some(t) => format!("{}NITF::UDHOFL: {}\n", retval, &t),
            None => retval,
        };
        retval = match &self.udhd {
            Some(t) => {
                for (key, value) in &*t {
                    retval = format!("{}NITF::UDHD::{}\n", retval, key);
                }
                retval
            }
            None => retval,
        };
        retval = format!("{}NITF::XHDL: {}\n", retval, &self.xhdl);
        retval = match &self.xhdlofl {
            Some(t) => format!("{}NITF::XHDLOFL: {}\n", retval, &t),
            None => retval,
        };
        retval = match &self.xhd {
            Some(t) => {
                for (key, value) in &*t {
                    retval = format!("{}NITF::XHD::{}\n", retval, key);
                }
                retval
            }
            None => retval,
        };
        write!(f, "{}", retval)
    }
}


fn nitf_parser(input: &[u8]) -> IResult<&[u8], Nitf> {
    let (input, header) = nitf_file_header_parser (input)?;

    let mut image_subheaders = Vec::new();

    // for is in 0..header.numi {
    //     let (input, im_subheader) = nitf_image_subheader_parser (input)?;
    //     image_subheaders.push(im_subheader);
    // }
    Ok((input, Nitf {
        nitf_file_header: header,
        nitf_image_subheader: image_subheaders,
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
    let (input, fsdctp) = take(2 as u64)(input)?;
    let (input, fsdcdt) = take(8 as u64)(input)?;
    let (input, fsdcxm) = take(4 as u64)(input)?;
    let (input, fsdg) = one_of(" SCR")(input)?;
    let (input, fsdgdt) = take(8 as u64)(input)?;
    let (input, fsctlx) = take(43 as u64)(input)?;
    let (input, fscatp) = one_of(" DOM")(input)?;
    let (input, fscaut) = take(40 as u64)(input)?;
    let (input, fscrsn) = one_of(" ABCDEFGH")(input)?;
    let (input, fssrdt) = take(8 as u64)(input)?;
    let (input, fsctln) = take(15 as u64)(input)?;
    let (input, fscop) = take(5 as u64)(input)?;
    let (input, fscpys) = take(5 as u64)(input)?;
    let (input, encryp) = take(1 as u64)(input)?;
    let (input, fbkgc_r) = take(1 as u64)(input)?;
    let (input, fbkgc_g) = take(1 as u64)(input)?;
    let (input, fbkgc_b) = take(1 as u64)(input)?;
    let (input, oname) = take(24 as u64)(input)?;
    let (input, ophone) = take(18 as u64)(input)?;
    let (input, fl) = take(12 as u64)(input)?;
    let (input, hl) = take(6 as u64)(input)?;

    let (input, numi) = take(3 as u64)(input)?;

    let mut lish = Vec::new();
    let mut li = Vec::new();
    for is in 0 .. usize::from_str(std::str::from_utf8(&numi).unwrap()).unwrap() {
        let (input, lish_t) = take(6 as u64)(input)?;
        lish.push(usize::from_str(std::str::from_utf8(&lish_t).unwrap()).unwrap());
        let (input, li_t) = take(10 as u64)(input)?;
        li.push(usize::from_str(std::str::from_utf8(&li_t).unwrap()).unwrap());
    }
    let (input, _) = take(usize::from_str(std::str::from_utf8(&numi).unwrap()).unwrap()*16)(input)?;

    let (input, nums) = take(3 as u64)(input)?;

    let mut lssh = Vec::new();
    let mut ls = Vec::new();
    for gs in 0 .. usize::from_str(std::str::from_utf8(&nums).unwrap()).unwrap() {
        let (input, lssh_t) = take(4 as u64)(input)?;
        lssh.push(usize::from_str(std::str::from_utf8(&lssh_t).unwrap()).unwrap());
        let (input, ls_t) = take(6 as u64)(input)?;
        ls.push(usize::from_str(std::str::from_utf8(&ls_t).unwrap()).unwrap());
    }
    let (input, _) = take(usize::from_str(std::str::from_utf8(&nums).unwrap()).unwrap()*10)(input)?;

    let (input, numx) = take(3 as u64)(input)?;

    let (input, numt) = take(3 as u64)(input)?;

    let mut ltsh = Vec::new();
    let mut lt = Vec::new();
    for ts in 0 .. usize::from_str(std::str::from_utf8(&numt).unwrap()).unwrap() {
        let (input, ltsh_t) = take(4 as u64)(input)?;
        ltsh.push(usize::from_str(std::str::from_utf8(&ltsh_t).unwrap()).unwrap());
        let (input, lt_t) = take(5 as u64)(input)?;
        lt.push(usize::from_str(std::str::from_utf8(&lt_t).unwrap()).unwrap());
    }
    let (input, _) = take(usize::from_str(std::str::from_utf8(&numt).unwrap()).unwrap()*9)(input)?;

    let (input, numdes) = take(3 as u64)(input)?;

    let mut ldsh = Vec::new();
    let mut ld = Vec::new();
    for des in 0 .. usize::from_str(std::str::from_utf8(&numdes).unwrap()).unwrap() {
        let (input, ldsh_t) = take(4 as u64)(input)?;
        ldsh.push(usize::from_str(std::str::from_utf8(&ldsh_t).unwrap()).unwrap());
        let (input, ld_t) = take(9 as u64)(input)?;
        ld.push(usize::from_str(std::str::from_utf8(&ld_t).unwrap()).unwrap());
    }
    let (input, _) = take(usize::from_str(std::str::from_utf8(&numdes).unwrap()).unwrap()*13)(input)?;

    let (input, numres) = take(3 as u64)(input)?;

    let mut lresh = Vec::new();
    let mut lre = Vec::new();
    for res in 0 .. usize::from_str(std::str::from_utf8(&numres).unwrap()).unwrap() {
        let (input, lresh_t) = take(4 as u64)(input)?;
        lresh.push(usize::from_str(std::str::from_utf8(&lresh_t).unwrap()).unwrap());
        let (input, lre_t) = take(7 as u64)(input)?;
        lre.push(usize::from_str(std::str::from_utf8(&lre_t).unwrap()).unwrap());
    }

    let (input, _) = take(usize::from_str(std::str::from_utf8(&numres).unwrap()).unwrap()*11)(input)?;

    let mut udhofl= None;
    let mut udhd = None;

    let (input, udhdl) = take(5 as u64)(input)?;
    if usize::from_str(std::str::from_utf8(&udhdl).unwrap()).unwrap() > 0 {
        let (input, udhofl_t) = take(3 as u64)(input)?;
        udhofl = Some(usize::from_str(std::str::from_utf8(&udhofl_t).unwrap()).unwrap());
        let mut udhd_map = HashMap::new();
        let mut pointer = 3 as usize;
        while pointer < udhofl.unwrap() {
            let (input, (size,key,value)) = nitf_tre_parser(input)?;
            udhd_map.insert(key, value);
            pointer += size;
        }
        udhd = Some(udhd_map);
    }

    let mut xhdlofl = None;
    let mut xhd = None;

    let (input, xhdl) = take(5 as u64)(input)?;
    if usize::from_str(std::str::from_utf8(&xhdl).unwrap()).unwrap() > 0 {
        let (input, xhdlofl_t) = take(3 as u64)(input)?;
        xhdlofl = Some(usize::from_str(std::str::from_utf8(&xhdlofl_t).unwrap()).unwrap());
        let mut xhd_map = HashMap::new();
        let mut pointer = 3 as usize;
        while pointer < xhdlofl.unwrap() {
            let (input, (size,key,value)) = nitf_tre_parser(input)?;
            xhd_map.insert(key, value);
            pointer += size;
        }
        xhd = Some(xhd_map);
    }

    Ok((input, NitfFileHeader {
        fhdr:   std::str::from_utf8(&fhdr).unwrap().parse().unwrap(),
        fver:   std::str::from_utf8(&fver).unwrap().parse().unwrap(),
        clevel: u8::from_str(std::str::from_utf8(&clevel).unwrap()).unwrap(),
        stype:  std::str::from_utf8(&stype).unwrap().parse().unwrap(),
        ostaid: std::str::from_utf8(&ostaid).unwrap().trim().parse().unwrap(),
        fdt:    DateTime::<Utc>::from_utc(
                                    NaiveDateTime::parse_from_str(std::str::from_utf8(&fdt)
                                                        .unwrap(),
                                    "%Y%m%d%H%M%S").unwrap(), Utc
                    ),
        ftitle: std::str::from_utf8(&ftitle).unwrap().trim().parse().unwrap(),
        fsclas,
        fsclsy: std::str::from_utf8(&fsclsy).unwrap().trim().parse().unwrap(),
        fscode: std::str::from_utf8(&fscode).unwrap().trim().parse().unwrap(),
        fsctlh: std::str::from_utf8(&fsctlh).unwrap().trim().parse().unwrap(),
        fsrel:  std::str::from_utf8(&fsrel).unwrap().trim().parse().unwrap(),
        fsdctp: std::str::from_utf8(&fsdctp).unwrap().trim().parse().unwrap(),
        fsdcdt: match std::str::from_utf8(&fsdcdt).unwrap().parse::<String>().unwrap().trim() {
            "" => None,
            _ => Some(Date::<Utc>::from_utc(
                NaiveDate::parse_from_str(std::str::from_utf8(&fsdcdt)
                                              .unwrap(),
                                          "%Y%m%d").unwrap(), Utc
            )),
        },
        fsdcxm: std::str::from_utf8(&fsdcxm).unwrap().trim().parse().unwrap(),
        fsdg,
        fsdgdt: match std::str::from_utf8(&fsdgdt).unwrap().parse::<String>().unwrap().trim() {
            "" => None,
            _ => Some(Date::<Utc>::from_utc(
                NaiveDate::parse_from_str(std::str::from_utf8(&fsdgdt)
                                              .unwrap(),
                                          "%Y%m%d").unwrap(), Utc
            )),
        },
        fsctlx: std::str::from_utf8(&fsctlx).unwrap().trim().parse().unwrap(),
        fscatp,
        fscaut: std::str::from_utf8(&fscaut).unwrap().trim().parse().unwrap(),
        fscrsn,
        fssrdt: match std::str::from_utf8(&fssrdt).unwrap().parse::<String>().unwrap().trim() {
            "" => None,
            _ => Some(Date::<Utc>::from_utc(
                NaiveDate::parse_from_str(std::str::from_utf8(&fssrdt)
                                              .unwrap(),
                                          "%Y%m%d").unwrap(), Utc
            )),
        },
        fsctln: std::str::from_utf8(&fsctln).unwrap().trim().parse().unwrap(),
        fscop: u32::from_str(std::str::from_utf8(&fscop).unwrap()).unwrap(),
        fscpys: u32::from_str(std::str::from_utf8(&fscpys).unwrap()).unwrap(),
        encryp: u8::from_str(std::str::from_utf8(&encryp).unwrap()).unwrap(),
        fbkgc_r: fbkgc_r[0],
        fbkgc_g: fbkgc_g[0],
        fbkgc_b: fbkgc_b[0],
        oname: std::str::from_utf8(&oname).unwrap().trim().parse().unwrap(),
        ophone: std::str::from_utf8(&ophone).unwrap().trim().parse().unwrap(),
        fl: usize::from_str(std::str::from_utf8(&fl).unwrap()).unwrap(),
        hl: usize::from_str(std::str::from_utf8(&hl).unwrap()).unwrap(),
        numi: usize::from_str(std::str::from_utf8(&numi).unwrap()).unwrap(),
        lish,
        li,
        nums: usize::from_str(std::str::from_utf8(&nums).unwrap()).unwrap(),
        lssh,
        ls,
        numx: usize::from_str(std::str::from_utf8(&numx).unwrap()).unwrap(),
        numt: usize::from_str(std::str::from_utf8(&numt).unwrap()).unwrap(),
        ltsh,
        lt,
        numdes: usize::from_str(std::str::from_utf8(&numdes).unwrap()).unwrap(),
        ldsh,
        ld,
        numres: usize::from_str(std::str::from_utf8(&numres).unwrap()).unwrap(),
        lresh,
        lre,
        udhdl: usize::from_str(std::str::from_utf8(&udhdl).unwrap()).unwrap(),
        udhofl,
        udhd,
        xhdl: usize::from_str(std::str::from_utf8(&xhdl).unwrap()).unwrap(),
        xhdlofl,
        xhd,

    }))
}


fn nitf_image_subheader_parser(input: &[u8]) -> IResult<&[u8], NitfImageSubheader> {
    let (input, im) = tag("IM")(input)?;
    let (input, iid1) = take(10 as u64)(input)?;
    let (input, idatim) = take(14 as u64)(input)?;
    Ok((input, NitfImageSubheader {
        im: std::str::from_utf8(&im).unwrap().parse().unwrap(),
        iid1: std::str::from_utf8(&iid1).unwrap().parse().unwrap(),
        idatim: DateTime::<Utc>::from_utc(
            NaiveDateTime::parse_from_str(std::str::from_utf8(&idatim)
                                              .unwrap(),
                                          "%Y%m%d%H%M%S").unwrap(), Utc
        ),
    }))
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