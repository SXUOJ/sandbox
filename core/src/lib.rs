pub mod child;
pub mod config;
pub mod error;
mod killer;
pub mod result;
pub mod runner;

pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(target_os = "linux")]
mod seccomp;
