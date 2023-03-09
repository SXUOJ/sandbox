mod child;
mod cmp;
mod config;
mod error;
mod killer;
mod result;
mod runner;

pub use cmp::{try_compare, ByteReader, UnixFdReader};
pub use config::{Config, Langs};
pub use error::Error;
pub use result::{infer_result, JudgeResult, Status};
pub use runner::run;

pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(target_os = "linux")]
mod seccomp;
