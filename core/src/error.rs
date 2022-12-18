use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(": {0}")]
    NixError(#[from] nix::errno::Errno),
    #[error(": {0}")]
    IOError(#[from] std::io::Error),
    #[error(": {0}")]
    FFINulError(#[from] std::ffi::NulError),

    #[cfg(target_os = "linux")]
    #[error(": {0}")]
    SeccompError(#[from] libseccomp::error::SeccompError),
}
