use netcdf::attribute::Attribute;
use netcdf::extent::{Extents, Extents::All};
use netcdf::types::{BasicType, VariableType};
use rustler::{Env, Term};

mod error;
mod types;

use error::NetCDFError;
use types::file::{NetCDFFile, NetCDFFileRef};
use types::value::Value;
use types::variable::NetCDFVariable;

rustler::atoms! {
    nil,
    ok,
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
    string_t = "string"
}

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(NetCDFFileRef, env);
    true
}

#[rustler::nif]
fn file_open(filename: &str) -> Result<NetCDFFile, NetCDFError> {
    let filepath = std::path::Path::new(filename);
    let file = netcdf::open(filepath)?;
    Ok(NetCDFFile::new(file, filename, Vec::<String>::new()))
}

#[rustler::nif]
fn file_variables(ex_file: NetCDFFile) -> Result<Vec<String>, NetCDFError> {
    let file = &ex_file.resource.0;
    let result = file.variables().map(|var| var.name()).collect();
    Ok(result)
}

#[rustler::nif]
fn file_open_with_variables(filename: &str) -> Result<NetCDFFile, NetCDFError> {
    let filepath = std::path::Path::new(filename);
    let file = netcdf::open(filepath)?;
    let variables = file.variables().map(|var| var.name()).collect();
    Ok(NetCDFFile::new(file, filename, variables))
}

#[rustler::nif]
fn variable_values(
    ex_file: NetCDFFile,
    variable_name: &str,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    let file = &ex_file.resource.0;

    file.variable(variable_name)
        .ok_or(NetCDFError::NotFound())
        .and_then(|var| get_variable_values(&var))
}

fn get_variable_values(
    variable: &netcdf::variable::Variable,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    match variable.vartype() {
        VariableType::Basic(BasicType::Byte) => load_numeric_variable_values::<u8>(variable),
        VariableType::Basic(BasicType::Char) => load_numeric_variable_values::<i8>(variable),
        VariableType::Basic(BasicType::Ubyte) => load_numeric_variable_values::<u8>(variable),
        VariableType::Basic(BasicType::Short) => load_numeric_variable_values::<i16>(variable),
        VariableType::Basic(BasicType::Ushort) => load_numeric_variable_values::<u16>(variable),
        VariableType::Basic(BasicType::Int) => load_numeric_variable_values::<i32>(variable),
        VariableType::Basic(BasicType::Uint) => load_numeric_variable_values::<u32>(variable),
        VariableType::Basic(BasicType::Int64) => load_numeric_variable_values::<i64>(variable),
        VariableType::Basic(BasicType::Uint64) => load_numeric_variable_values::<u64>(variable),
        VariableType::Basic(BasicType::Float) => load_numeric_variable_values::<f32>(variable),
        VariableType::Basic(BasicType::Double) => load_numeric_variable_values::<f64>(variable),
        VariableType::String => load_string_variable_values(variable),
        _ => Err(error::NetCDFError::NetCDF(
            netcdf::error::Error::WrongDataset,
        )),
    }
}

fn load_numeric_variable_values<T>(
    variable: &netcdf::variable::Variable,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError>
where
    Value: From<Vec<T>>,
    T: netcdf::NcPutGet,
{
    let var_type = variable.vartype();
    let type_name = var_type.name();
    let error = netcdf::error::Error::Str(format!("unable to load type {}", type_name));

    let value = variable
        .values::<T, Extents>(All)
        .map_err(|_| error::NetCDFError::NetCDF(error))?;
    Ok((Value::from(value), as_type_atom(&type_name)))
}

fn load_string_variable_values(
    variable: &netcdf::variable::Variable,
) -> Result<(Value, rustler::types::atom::Atom), NetCDFError> {
    let time_len = variable.len() as usize;
    let mut values: Vec<String> = Vec::with_capacity(time_len);

    for i in 0..time_len {
        let error = netcdf::error::Error::Str("unable to load string variable".to_string());
        let value: String = variable
            .string_value::<Extents>(i.into())
            .map_err(|_| error::NetCDFError::NetCDF(error))?;
        values.push(value);
    }

    Ok((Value::from(values), string_t()))
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
    ex_file: NetCDFFile,
    variable_name: &str,
) -> Result<Vec<(String, Value)>, NetCDFError> {
    let file = &ex_file.resource.0;
    file.variable(variable_name)
        .ok_or(NetCDFError::NotFound())
        .map(|var| get_variable_attributes(&var))
}

fn get_variable_attributes(variable: &netcdf::variable::Variable) -> Vec<(String, Value)> {
    variable
        .attributes()
        .map(parse_variable_attribute)
        .collect()
}

#[rustler::nif]
fn variable_load(ex_file: NetCDFFile, variable_name: &str) -> Result<NetCDFVariable, NetCDFError> {
    let file = &ex_file.resource.0;
    let variable = file
        .variable(variable_name)
        .ok_or(NetCDFError::NotFound())?;
    let (values, value_type) = get_variable_values(&variable)?;
    let attributes = get_variable_attributes(&variable);

    Ok(NetCDFVariable::new(
        variable_name.to_string(),
        values,
        value_type,
        attributes,
    ))
}

fn parse_variable_attribute(attr: Attribute) -> (String, Value) {
    let name = attr.name().to_string();
    let value = match attr.value() {
        Err(_) => Value::Atom(nil()),
        Ok(attr_value) => Value::from(attr_value),
    };

    (name, value)
}

rustler::init!(
    "Elixir.NetCDF.Native",
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
