use std::{ffi, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("NixErorr: {0}")]
    NixError(#[from] nix::errno::Errno),
    #[error("IOError: {0}")]
    IOError(#[from] io::Error),
    #[error("FFINulError: {0}")]
    FFINulError(#[from] ffi::NulError),
    #[error("CompareError: {0}")]
    CompareError(io::Error),

    #[cfg(target_os = "linux")]
    #[error(": {0}")]
    SeccompError(#[from] libseccomp::error::SeccompError),
}
