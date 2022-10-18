use netcdf::attribute;
use rustler::{Env, Term};

mod error;
mod types;

pub use error::NetCDFError;
pub use types::AttrValue;
pub use types::ExNetCDFFile;
pub use types::ExNetCDFFileRef;

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
    return Ok(result);
}

#[rustler::nif]
fn load_variable(
    ex_file: ExNetCDFFile,
    variable_name: &std::primitive::str,
) -> Result<Vec<f64>, NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = match file.variable(variable_name) {
        Some(var) => var,
        None => return Err(NetCDFError::NotFound()),
    };

    match variable.values::<f64>(None, None) {
        Ok(result) => return Ok(result.into_raw_vec()),
        Err(e) => return Err(NetCDFError::NetCDF(e)),
    };
}

#[rustler::nif]
fn get_variable_attributes(
    ex_file: ExNetCDFFile,
    variable_name: &std::primitive::str,
) -> Result<Vec<(String, AttrValue)>, NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = match file.variable(variable_name) {
        Some(var) => var,
        None => return Err(NetCDFError::NotFound()),
    };

    let result = variable
        .attributes()
        .map(parse_variable_attribute)
        .collect();

    return Ok(result);
}

fn parse_variable_attribute(attr: attribute::Attribute) -> (String, AttrValue) {
    let name = attr.name().to_string();
    let value = match attr.value() {
        Err(_e) => AttrValue::Atom(nil()),
        Ok(attr_value) => AttrValue::from(attr_value),
    };

    return (name, value);
}

rustler::init!(
    "Elixir.DataParser.NetCDF",
    [
        open_file,
        get_file_variables,
        load_variable,
        get_variable_attributes
    ],
    load = on_load
);
