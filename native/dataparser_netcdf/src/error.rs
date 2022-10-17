use rustler::{Encoder, Env, Term};
use thiserror::Error;

rustler::atoms! {
    ok,
    error
}

#[derive(Error, Debug)]
pub enum NetCDFError {
    #[error("NetCDF Error")]
    NetCDF(#[from] netcdf::error::Error),
    #[error("Internal Error: {0}")]
    Internal(String),
    #[error("Other error: {0}")]
    Other(String),
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl Encoder for NetCDFError {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        format!("{:?}", self).encode(env)
    }
}
