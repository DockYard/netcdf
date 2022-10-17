use rustler::{Env, NifStruct, ResourceArc, Term};
mod error;
pub use error::NetCDFError;

pub struct ExNetCDFFileRef(pub netcdf::file::File);

#[derive(NifStruct)]
#[module = "DataParser.ExNetCDFFile"]
pub struct ExNetCDFFile {
    pub resource: ResourceArc<ExNetCDFFileRef>
}

impl ExNetCDFFileRef {
    pub fn new(file: netcdf::file::File) -> Self {
        Self(file)
    }
}

impl ExNetCDFFile {
    pub fn new(file: netcdf::file::File) -> Self {
        Self {
            resource: ResourceArc::new(ExNetCDFFileRef::new(file)),
        }
    }
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
    return Ok(ExNetCDFFile::new(file));
}

#[rustler::nif]
fn get_file_variables(ex_file: ExNetCDFFile) -> Result<Vec<std::string::String>, NetCDFError> {
    let file = &ex_file.resource.0;
    let result = file.variables().map(|var| var.name()).collect();
    Ok(result)
}

#[rustler::nif]
fn load_variable(ex_file: ExNetCDFFile, variable_name: &std::primitive::str) -> Result<ArrayD<Numeric>, NetCDFError> {
    let file = &ex_file.resource.0;
    let result = file.variable(variable_name).values();
    Ok(result)
}

rustler::init!("Elixir.DataParser.NetCDF", [open_file, get_file_variables, load_variable], load=on_load);
