use nix::errno::Errno;
use std::ffi::NulError;
use std::io;

#[derive(Debug)]
pub enum CoreError {
    NixErrno(Errno),
    FFINulError(NulError),
    IOError(io::Error),
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
