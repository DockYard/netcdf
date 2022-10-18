use netcdf::attribute::AttrValue;
use netcdf::file::File;
use rustler::{Env, NifStruct, ResourceArc, Term};

pub struct ExNetCDFFileRef(pub File);

rustler::atoms! {
    nil
}

#[derive(NifStruct)]
#[module = "DataParser.NetCDF.File"]
pub struct ExNetCDFFile {
    pub resource: ResourceArc<ExNetCDFFileRef>,
    pub filename: String,
}

impl ExNetCDFFileRef {
    pub fn new(file: File) -> Self {
        Self(file)
    }
}

impl ExNetCDFFile {
    pub fn new(file: File, filename: &str) -> Self {
        Self {
            resource: ResourceArc::new(ExNetCDFFileRef::new(file)),
            filename: filename.to_string(),
        }
    }
}

pub enum Value {
    Atom(rustler::types::atom::Atom),
    Uchar(u8),
    Schar(i8),
    Ushort(u16),
    Short(i16),
    Uint(u32),
    Int(i32),
    Ulonglong(u64),
    Longlong(i64),
    Float(f32),
    Double(f64),
    Str(String),
    Uchars(Vec<u8>),
    Schars(Vec<i8>),
    Ushorts(Vec<u16>),
    Shorts(Vec<i16>),
    Uints(Vec<u32>),
    Ints(Vec<i32>),
    Ulonglongs(Vec<u64>),
    Longlongs(Vec<i64>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    Strs(Vec<String>),
}

impl rustler::Encoder for Value {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let result = match self {
            Self::Atom(atom) => atom.encode(env),
            Self::Uchar(x) => x.encode(env),
            Self::Schar(x) => x.encode(env),
            Self::Ushort(x) => x.encode(env),
            Self::Short(x) => x.encode(env),
            Self::Uint(x) => x.encode(env),
            Self::Int(x) => x.encode(env),
            Self::Ulonglong(x) => x.encode(env),
            Self::Longlong(x) => x.encode(env),
            Self::Float(x) => x.encode(env),
            Self::Double(x) => x.encode(env),
            Self::Str(x) => x.encode(env),
            Self::Uchars(x) => x.encode(env),
            Self::Schars(x) => x.encode(env),
            Self::Ushorts(x) => x.encode(env),
            Self::Shorts(x) => x.encode(env),
            Self::Uints(x) => x.encode(env),
            Self::Ints(x) => x.encode(env),
            Self::Ulonglongs(x) => x.encode(env),
            Self::Longlongs(x) => x.encode(env),
            Self::Floats(x) => x.encode(env),
            Self::Doubles(x) => x.encode(env),
            Self::Strs(x) => x.encode(env),
        };
        return result;
    }
}

impl From<AttrValue> for Value {
    fn from(value: AttrValue) -> Value {
        match value {
            AttrValue::Uchar(value) => Self::Uchar(value),
            AttrValue::Schar(value) => Self::Schar(value),
            AttrValue::Ushort(value) => Self::Ushort(value),
            AttrValue::Short(value) => Self::Short(value),
            AttrValue::Uint(value) => Self::Uint(value),
            AttrValue::Int(value) => Self::Int(value),
            AttrValue::Ulonglong(value) => Self::Ulonglong(value),
            AttrValue::Longlong(value) => Self::Longlong(value),
            AttrValue::Float(value) => Self::Float(value),
            AttrValue::Double(value) => Self::Double(value),
            AttrValue::Str(value) => Self::Str(value),
            AttrValue::Uchars(value) => Self::Uchars(value),
            AttrValue::Schars(value) => Self::Schars(value),
            AttrValue::Ushorts(value) => Self::Ushorts(value),
            AttrValue::Shorts(value) => Self::Shorts(value),
            AttrValue::Uints(value) => Self::Uints(value),
            AttrValue::Ints(value) => Self::Ints(value),
            AttrValue::Ulonglongs(value) => Self::Ulonglongs(value),
            AttrValue::Longlongs(value) => Self::Longlongs(value),
            AttrValue::Floats(value) => Self::Floats(value),
            AttrValue::Doubles(value) => Self::Doubles(value),
            AttrValue::Strs(value) => Self::Strs(value),
        }
    }
}
