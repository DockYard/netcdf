use netcdf::attribute::AttrValue;
use rustler::types::atom::Atom;
use rustler::{Env, Term};

pub enum Value {
    Atom(Atom),
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

impl<'a> rustler::Decoder<'a> for Value {
    fn decode(_term: Term<'a>) -> rustler::NifResult<Self> {
        Err(rustler::Error::RaiseAtom("unable to decode"))
    }
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

        result
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

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Value {
        Self::Uchars(value)
    }
}

impl From<Vec<i8>> for Value {
    fn from(value: Vec<i8>) -> Value {
        Self::Schars(value)
    }
}

impl From<Vec<u16>> for Value {
    fn from(value: Vec<u16>) -> Value {
        Self::Ushorts(value)
    }
}

impl From<Vec<i16>> for Value {
    fn from(value: Vec<i16>) -> Value {
        Self::Shorts(value)
    }
}

impl From<Vec<u32>> for Value {
    fn from(value: Vec<u32>) -> Value {
        Self::Uints(value)
    }
}

impl From<Vec<i32>> for Value {
    fn from(value: Vec<i32>) -> Value {
        Self::Ints(value)
    }
}

impl From<Vec<u64>> for Value {
    fn from(value: Vec<u64>) -> Value {
        Self::Ulonglongs(value)
    }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Value {
        Self::Longlongs(value)
    }
}

impl From<Vec<f32>> for Value {
    fn from(value: Vec<f32>) -> Value {
        Self::Floats(value)
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Value {
        Self::Doubles(value)
    }
}

impl From<Vec<String>> for Value {
    fn from(value: Vec<String>) -> Value {
        Self::Strs(value)
    }
}
