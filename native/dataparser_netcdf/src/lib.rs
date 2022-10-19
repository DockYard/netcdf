use netcdf::attribute::Attribute;
use netcdf::types::{BasicType, VariableType};
use rustler::{Env, Term};

mod error;
mod types;

pub use error::NetCDFError;
pub use types::ExNetCDFFile;
pub use types::ExNetCDFFileRef;
pub use types::ExNetCDFVariable;
pub use types::Value;

rustler::atoms! {
    nil,
    non_numeric,
    i8_t = "i8",
    i16_t = "i16",
    i32_t = "i32",
    i64_t = "i64",
    u8_t = "u8",
    u16_t = "u16",
    u32_t = "u32",
    u64_t = "u64",
    f32_t = "f32",
    f64_t = "f64",
}

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExNetCDFFileRef, env);
    true
}

#[rustler::nif]
fn file_open(filename: &str) -> Result<ExNetCDFFile, NetCDFError> {
    let filepath = std::path::Path::new(filename);
    let file = match netcdf::open(filepath) {
        Ok(file) => file,
        Err(e) => return Err(NetCDFError::NetCDF(e)),
    };
    return Ok(ExNetCDFFile::new(file, filename, Vec::<String>::new()));
}

#[rustler::nif]
fn file_variables(ex_file: ExNetCDFFile) -> Result<Vec<String>, NetCDFError> {
    let file = &ex_file.resource.0;
    let result = file.variables().map(|var| var.name()).collect();
    return Ok(result);
}

#[rustler::nif]
fn file_open_with_variables(filename: &str) -> Result<ExNetCDFFile, NetCDFError> {
    let filepath = std::path::Path::new(filename);
    let file = match netcdf::open(filepath) {
        Ok(file) => file,
        Err(e) => return Err(NetCDFError::NetCDF(e)),
    };
    let variables = file.variables().map(|var| var.name()).collect();
    return Ok(ExNetCDFFile::new(file, filename, variables));
}

#[rustler::nif]
fn variable_values(
    ex_file: ExNetCDFFile,
    variable_name: &str,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = match file.variable(variable_name) {
        Some(var) => var,
        None => return Err(NetCDFError::NotFound()),
    };

    return get_variable_values(&variable);
}

fn get_variable_values(
    variable: &netcdf::variable::Variable,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    let var_type = variable.vartype();
    let type_name = var_type.name();

    let values = match var_type {
        VariableType::Basic(BasicType::Byte) => variable
            .values::<u8>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Char) => variable
            .values::<i8>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Ubyte) => variable
            .values::<u8>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Short) => variable
            .values::<i16>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Ushort) => variable
            .values::<u16>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Int) => variable
            .values::<i32>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Uint) => variable
            .values::<u32>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Int64) => variable
            .values::<i64>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Uint64) => variable
            .values::<u64>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Float) => variable
            .values::<f32>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        VariableType::Basic(BasicType::Double) => variable
            .values::<f64>(None, None)
            .map(|x| Value::from(x.into_raw_vec())),
        _ => Err(netcdf::error::Error::Str(format!(
            "unable to load type {}",
            type_name
        ))),
    };

    match values {
        Ok(result) => return Ok((result, as_type_atom(&type_name))),
        Err(e) => return Err(NetCDFError::NetCDF(e)),
    };
}

fn as_type_atom(type_name: &str) -> rustler::types::atom::Atom {
    match type_name {
        "i8" => i8_t(),
        "i16" => i16_t(),
        "i32" => i32_t(),
        "i64" => i64_t(),
        "u8" => u8_t(),
        "u16" => u16_t(),
        "u32" => u32_t(),
        "u64" => u64_t(),
        "f32" => f32_t(),
        "f64" => f64_t(),
        _ => non_numeric(),
    }
}

#[rustler::nif]
fn variable_attributes(
    ex_file: ExNetCDFFile,
    variable_name: &str,
) -> Result<Vec<(String, Value)>, NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = match file.variable(variable_name) {
        Some(var) => var,
        None => return Err(NetCDFError::NotFound()),
    };

    return Ok(get_variable_attributes(&variable));
}

fn get_variable_attributes(variable: &netcdf::variable::Variable) -> Vec<(String, Value)> {
    return variable
        .attributes()
        .map(parse_variable_attribute)
        .collect();
}

#[rustler::nif]
fn variable_load(
    ex_file: ExNetCDFFile,
    variable_name: &str,
) -> Result<ExNetCDFVariable, NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = match file.variable(variable_name) {
        Some(var) => var,
        None => return Err(NetCDFError::NotFound()),
    };

    let values = match get_variable_values(&variable) {
        Ok(values) => values,
        Err(e) => return Err(e),
    };

    let attributes = get_variable_attributes(&variable);

    return Ok(ExNetCDFVariable::new(
        variable_name.to_string(),
        values.0,
        values.1,
        attributes,
    ));
}

fn parse_variable_attribute(attr: Attribute) -> (String, Value) {
    let name = attr.name().to_string();
    let value = match attr.value() {
        Err(_e) => Value::Atom(nil()),
        Ok(attr_value) => Value::from(attr_value),
    };

    return (name, value);
}

rustler::init!(
    "Elixir.DataParser.NetCDF.Native",
    [
        file_open,
        file_variables,
        file_open_with_variables,
        variable_load,
        variable_values,
        variable_attributes
    ],
    load = on_load
);
