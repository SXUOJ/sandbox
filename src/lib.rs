mod core;
mod grpc;
mod judger;
mod langs;

pub use crate::core::{
    infer_result, run, try_compare, ByteReader, Config, Error, JudgeResult, Langs, Status,
    UnixFdReader,
};
pub use grpc::{judger as grpc_judger, MyJudger};
