use rustler::{Env, Term};
use netcdf::attribute;

mod error;
mod types;

pub use types::AttrValue;
pub use types::ExNetCDFFile;
pub use types::ExNetCDFFileRef;
pub use error::NetCDFError;

rustler::atoms! {
    nil
}

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExNetCDFFileRef, env);
    true
}

#[rustler::nif]
fn open_file(filename: &std::primitive::str) -> Result<ExNetCDFFile, NetCDFError> {
    let filepath = std::path::Path::new(filename);
    let file = match netcdf::open(filepath) {
        Ok(file) => file,
        Err(e) => return Err(NetCDFError::NetCDF(e)),
    };
    return Ok(ExNetCDFFile::new(file, filename));
}

#[rustler::nif]
fn get_file_variables(ex_file: ExNetCDFFile) -> Result<Vec<String>, NetCDFError> {
    let file = &ex_file.resource.0;
    let result = file.variables().map(|var| var.name()).collect();
    return Ok(result)
}

#[rustler::nif]
fn load_variable(ex_file: ExNetCDFFile, variable_name: &std::primitive::str) -> Result<Vec<f64>, NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = file.variable(variable_name).unwrap();

    match variable.values::<f64>(None, None) {
        Ok(result) => return Ok(result.into_raw_vec()),
        Err(e) => return Err(NetCDFError::NetCDF(e))
    };
}

#[rustler::nif]
fn get_variable_attributes(ex_file: ExNetCDFFile, variable_name: &std::primitive::str) -> Result<Vec<(String, AttrValue)>, NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = file.variable(variable_name).unwrap();
    let result = variable.attributes().map(parse_variable_attribute).collect();
    return Ok(result);
}

fn parse_variable_attribute(attr: attribute::Attribute) -> (String, AttrValue) {
    let name = attr.name().to_string();
    let value = match attr.value() {
        Err(_e) => AttrValue::Atom(nil()),
        Ok(attribute::AttrValue::Uchar(value)) => AttrValue::Uchar(value),
        Ok(attribute::AttrValue::Schar(value)) => AttrValue::Schar(value),
        Ok(attribute::AttrValue::Ushort(value)) => AttrValue::Ushort(value),
        Ok(attribute::AttrValue::Short(value)) => AttrValue::Short(value),
        Ok(attribute::AttrValue::Uint(value)) => AttrValue::Uint(value),
        Ok(attribute::AttrValue::Int(value)) => AttrValue::Int(value),
        Ok(attribute::AttrValue::Ulonglong(value)) => AttrValue::Ulonglong(value),
        Ok(attribute::AttrValue::Longlong(value)) => AttrValue::Longlong(value),
        Ok(attribute::AttrValue::Float(value)) => AttrValue::Float(value),
        Ok(attribute::AttrValue::Double(value)) => AttrValue::Double(value),
        Ok(attribute::AttrValue::Str(value)) => AttrValue::Str(value),
        Ok(attribute::AttrValue::Uchars(value)) => AttrValue::Uchars(value),
        Ok(attribute::AttrValue::Schars(value)) => AttrValue::Schars(value),
        Ok(attribute::AttrValue::Ushorts(value)) => AttrValue::Ushorts(value),
        Ok(attribute::AttrValue::Shorts(value)) => AttrValue::Shorts(value),
        Ok(attribute::AttrValue::Uints(value)) => AttrValue::Uints(value),
        Ok(attribute::AttrValue::Ints(value)) => AttrValue::Ints(value),
        Ok(attribute::AttrValue::Ulonglongs(value)) => AttrValue::Ulonglongs(value),
        Ok(attribute::AttrValue::Longlongs(value)) => AttrValue::Longlongs(value),
        Ok(attribute::AttrValue::Floats(value)) => AttrValue::Floats(value),
        Ok(attribute::AttrValue::Doubles(value)) => AttrValue::Doubles(value),
        Ok(attribute::AttrValue::Strs(value)) => AttrValue::Strs(value),
    };

    return (name, value)
}


rustler::init!("Elixir.DataParser.NetCDF", [open_file, get_file_variables, load_variable, get_variable_attributes], load=on_load);
