use rustler::{Env, NifStruct, ResourceArc, Term};
use netcdf::file::File;

pub struct ExNetCDFFileRef(pub File);

rustler::atoms! {
    nil
}

#[derive(NifStruct)]
#[module = "DataParser.NetCDF.File"]
pub struct ExNetCDFFile {
    pub resource: ResourceArc<ExNetCDFFileRef>,
    pub filename: String
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
            filename: filename.to_string()
        }
    }
}

pub enum AttrValue {
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

impl rustler::Encoder for AttrValue {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let result = match self {
            AttrValue::Atom(atom) => atom.encode(env),
            AttrValue::Uchar(x) => x.encode(env),
            AttrValue::Schar(x) => x.encode(env),
            AttrValue::Ushort(x) => x.encode(env),
            AttrValue::Short(x) => x.encode(env),
            AttrValue::Uint(x) => x.encode(env),
            AttrValue::Int(x) => x.encode(env),
            AttrValue::Ulonglong(x) => x.encode(env),
            AttrValue::Longlong(x) => x.encode(env),
            AttrValue::Float(x) => x.encode(env),
            AttrValue::Double(x) => x.encode(env),
            AttrValue::Str(x) => x.encode(env),
            AttrValue::Uchars(x) => x.encode(env),
            AttrValue::Schars(x) => x.encode(env),
            AttrValue::Ushorts(x) => x.encode(env),
            AttrValue::Shorts(x) => x.encode(env),
            AttrValue::Uints(x) => x.encode(env),
            AttrValue::Ints(x) => x.encode(env),
            AttrValue::Ulonglongs(x) => x.encode(env),
            AttrValue::Longlongs(x) => x.encode(env),
            AttrValue::Floats(x) => x.encode(env),
            AttrValue::Doubles(x) => x.encode(env),
            AttrValue::Strs(x) => x.encode(env),
        };
        return result
    }
}