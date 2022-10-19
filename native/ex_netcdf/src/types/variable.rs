use rustler::types::atom::Atom;
use rustler::NifStruct;

use crate::types::value::Value;

#[derive(NifStruct)]
#[module = "NetCDF.Variable"]
pub struct NetCDFVariable {
    pub name: String,
    pub value: Value,
    pub r#type: Atom,
    pub attributes: Vec<(String, Value)>,
}

impl NetCDFVariable {
    pub fn new(name: String, value: Value, t: Atom, attr: Vec<(String, Value)>) -> Self {
        Self {
            name: name,
            value: value,
            r#type: t,
            attributes: attr,
        }
    }
}
