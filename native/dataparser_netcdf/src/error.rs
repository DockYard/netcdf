use rustler::{Encoder, Env, Term};
use thiserror::Error;

rustler::atoms! {
    ok,
    error,
    not_found
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
    #[error("not_found")]
    NotFound(),
}

impl Encoder for NetCDFError {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            Self::NotFound() => not_found().encode(env),
            _ => format!("{:?}", self).encode(env),
        }
    }
}
