use netcdf::file::File;
use rustler::{NifStruct, ResourceArc};

pub struct NetCDFFileRef(pub File);

#[derive(NifStruct)]
#[module = "DataParser.NetCDF.File"]
pub struct NetCDFFile {
    pub resource: ResourceArc<NetCDFFileRef>,
    pub filename: String,
    pub variables: Vec<String>,
}

impl NetCDFFileRef {
    pub fn new(file: File) -> Self {
        Self(file)
    }
}

impl NetCDFFile {
    pub fn new(file: File, filename: &str, variables: Vec<String>) -> Self {
        Self {
            resource: ResourceArc::new(NetCDFFileRef::new(file)),
            filename: filename.to_string(),
            variables: variables,
        }
    }
}
