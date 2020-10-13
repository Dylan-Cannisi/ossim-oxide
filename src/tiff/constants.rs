use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BASELINE: HashMap<u16, IfdRegisteredTag> = {
        let mut map = HashMap::new();
        map.insert(315 as u16,
                          IfdRegisteredTag{ readable: "Artist", field_type: IfdType::Ascii});
        map.insert(258,   IfdRegisteredTag{ readable: "BitsPerSample", field_type: IfdType::Short});
        map.insert(265,   IfdRegisteredTag{ readable: "CellLength", field_type: IfdType::Short});
        map.insert(264,   IfdRegisteredTag{ readable: "CellWidth", field_type: IfdType::Short});
        map.insert(320,   IfdRegisteredTag{ readable: "ColorMap", field_type: IfdType::Short});
        map.insert(259,   IfdRegisteredTag{ readable: "Compression", field_type: IfdType::Short});
        map.insert(33432, IfdRegisteredTag{ readable: "Copyright", field_type: IfdType::Ascii});
        map.insert(306,   IfdRegisteredTag{ readable: "DateTime", field_type: IfdType::Ascii});
        map.insert(338,   IfdRegisteredTag{ readable: "ExtraSamples", field_type: IfdType::Short});
        map.insert(266,   IfdRegisteredTag{ readable: "FillOrder", field_type: IfdType::Short});
        map.insert(289,   IfdRegisteredTag{ readable: "FreeByteCounts", field_type: IfdType::Long});
        map.insert(288,   IfdRegisteredTag{ readable: "FreeOffsets", field_type: IfdType::Long});
        map.insert(291,   IfdRegisteredTag{ readable: "GrayResponseCurve", field_type: IfdType::Short});
        map.insert(290,   IfdRegisteredTag{ readable: "GrayResponseUnit", field_type: IfdType::Short});
        map.insert(316,   IfdRegisteredTag{ readable: "HostComputer", field_type: IfdType::Ascii});
        map.insert(270,   IfdRegisteredTag{ readable: "ImageDescription", field_type: IfdType::Ascii});
        map.insert(257,   IfdRegisteredTag{ readable: "ImageLength", field_type: IfdType::Undefined});
        map.insert(256,   IfdRegisteredTag{ readable: "ImageWidth", field_type: IfdType::Undefined});
        map.insert(271,   IfdRegisteredTag{ readable: "Make", field_type: IfdType::Ascii});
        map.insert(281,   IfdRegisteredTag{ readable: "MaxSampleValue", field_type: IfdType::Short});
        map.insert(280,   IfdRegisteredTag{ readable: "MinSampleValue", field_type: IfdType::Short});
        map.insert(272,   IfdRegisteredTag{ readable: "Model",  field_type: IfdType::Ascii});
        map.insert(254,   IfdRegisteredTag{ readable: "NewSubfileType", field_type: IfdType::Long});
        map.insert(274,   IfdRegisteredTag{ readable: "Orientation", field_type: IfdType::Short});
        map.insert(262,   IfdRegisteredTag{ readable: "PhotometricInterpretation", field_type: IfdType::Short});
        map.insert(284,   IfdRegisteredTag{ readable: "PlanarConfiguration", field_type: IfdType::Short});
        map.insert(296,   IfdRegisteredTag{ readable: "ResolutionUnit", field_type: IfdType::Short});
        map.insert(278,   IfdRegisteredTag{ readable: "RowsPerStrip", field_type: IfdType::Undefined});
        map.insert(277,   IfdRegisteredTag{ readable: "SamplesPerPixel", field_type: IfdType::Short});
        map.insert(305,   IfdRegisteredTag{ readable: "Software", field_type: IfdType::Ascii});
        map.insert(279,   IfdRegisteredTag{ readable: "StripByteCounts", field_type: IfdType::Undefined});
        map.insert(273,   IfdRegisteredTag{ readable: "StripOffsets", field_type: IfdType::Undefined});
        map.insert(255,   IfdRegisteredTag{ readable: "SubfileType", field_type: IfdType::Short});
        map.insert(263,   IfdRegisteredTag{ readable: "Threshholding", field_type: IfdType::Short});
        map.insert(282,   IfdRegisteredTag{ readable: "XResolution", field_type: IfdType::Rational});
        map.insert(283,   IfdRegisteredTag{ readable: "YResolution", field_type: IfdType::Rational});
        map
    };


    pub static ref GEOTIFF: HashMap<u16, IfdRegisteredTag> = {
        let mut map = HashMap::new();
        map.insert(34736 as u16,
                          IfdRegisteredTag{ readable: "GeoAsciiParams", field_type: IfdType::Short});
        map.insert(34737, IfdRegisteredTag{ readable: "GeoDoubleParams", field_type: IfdType::Short});
        map.insert(34735, IfdRegisteredTag{ readable: "GeoKeyDirectory", field_type: IfdType::Short});
        map.insert(33550, IfdRegisteredTag{ readable: "ModelPixelScale", field_type: IfdType::Short});
        map.insert(33922, IfdRegisteredTag{ readable: "ModelTiepoint", field_type: IfdType::Short});
        map.insert(34264, IfdRegisteredTag{ readable: "ModelTransformation", field_type: IfdType::Short});

        map
    };
}

pub struct IfdRegisteredTag {
    pub(crate) readable: &'static str,
    pub(crate) field_type: IfdType
}

pub enum IfdType {
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    SByte,
    Undefined,
    SShort,
    SLong,
    Float,
    Double,
}