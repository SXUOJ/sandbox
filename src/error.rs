use nix::errno::Errno;
use std::ffi::NulError;
use std::io;

#[cfg(target_os = "linux")]
use libseccomp::error::SeccompError;

#[derive(Debug)]
pub enum CoreError {
    NixErrno(Errno),
    FFINulError(NulError),
    IOError(io::Error),

    #[cfg(target_os = "linux")]
    SeccompError(SeccompError),
}

impl From<Errno> for CoreError {
    fn from(error: Errno) -> CoreError {
        CoreError::NixErrno(error)
    }
}

impl From<NulError> for CoreError {
    fn from(error: NulError) -> CoreError {
        CoreError::FFINulError(error)
    }
}

impl From<io::Error> for CoreError {
    fn from(error: io::Error) -> CoreError {
        CoreError::IOError(error)
    }
}

#[cfg(target_os = "linux")]
impl From<SeccompError> for CoreError {
    fn from(error: SeccompError) -> CoreError {
        CoreError::SeccompError(error)
    }
}
