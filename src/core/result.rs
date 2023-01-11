use nix::libc::{SIGSEGV, SIGUSR1, WEXITSTATUS, WIFSIGNALED, WTERMSIG};
use std::fs::read_to_string;

#[derive(Debug)]
pub struct JudgeResult {
    pub status: Status,
    pub signal: i32,
    pub real_time: f64,
    pub cpu_time: f64,
    pub memory: f64,
    pub error: String,
}

impl Default for JudgeResult {
    fn default() -> Self {
        Self {
            status: Status::Success,
            signal: 0,
            real_time: 0.0,
            cpu_time: 0.0,
            memory: 0.0,
            error: String::new(),
        }
    }
}

impl std::fmt::Display for JudgeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{status: {}, signal: {}, real_time: {}, cpu_time: {}, memory: {}, error: {}}}",
            self.status as i32, self.signal, self.real_time, self.cpu_time, self.memory, self.error,
        )
    }
}

pub fn infer_result(
    config: &crate::core::config::Config,
    raw_result: &crate::core::runner::RawJudgeResult,
) -> JudgeResult {
    let mut result = JudgeResult::default();

    result.real_time = raw_result.real_time_cost.as_millis() as f64;

    if WIFSIGNALED(raw_result.exit_status) {
        result.signal = WTERMSIG(raw_result.exit_status)
    }

    if result.signal == SIGUSR1 {
        result.status = Status::SystemError;
    } else {
        result.cpu_time = raw_result.resource_usage.ru_utime.tv_sec as f64 * 1000.0
            + raw_result.resource_usage.ru_utime.tv_usec as f64 / 1000.0;
        result.memory = raw_result.resource_usage.ru_maxrss as f64 / 1024.0;

        if WEXITSTATUS(raw_result.exit_status) != 0 {
            result.status = Status::RuntimeError;
        }

        if result.signal == SIGSEGV {
            if config.max_memory != 0 && result.memory > config.max_memory as f64 {
                result.status = Status::MemoryLimitExceed;
            } else {
                result.status = Status::RuntimeError;
            }
        } else {
            if result.signal != 0 {
                result.status = Status::RuntimeError;
            }

            if config.max_memory != 0 && result.memory > config.max_memory as f64 {
                result.status = Status::MemoryLimitExceed;
            }

            if config.real_time_limit != 0 && result.real_time > config.real_time_limit as f64 {
                result.status = Status::TimeLimitExceed;
            }

            if config.cpu_time_limit != 0 && result.cpu_time > config.cpu_time_limit as f64 {
                result.status = Status::TimeLimitExceed;
            }
        }
    };

    result.error = read_to_string(config.error_path.clone()).unwrap();

    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success = 0,
    Accepted = 1,
    WrongAnswer = 2,
    CompileError = 3,
    RuntimeError = 4,
    TimeLimitExceed = 5,
    MemoryLimitExceed = 6,
    OutputLimitExceed = 7,
    PresentationError = 8,
    SystemError = 9,
    UnkownError = 10,
}

impl From<u64> for Status {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Success,
            1 => Self::Accepted,
            2 => Self::WrongAnswer,
            3 => Self::CompileError,
            4 => Self::RuntimeError,
            5 => Self::TimeLimitExceed,
            6 => Self::MemoryLimitExceed,
            7 => Self::OutputLimitExceed,
            8 => Self::PresentationError,
            9 => Self::SystemError,
            _ => Self::UnkownError,
        }
    }
}

impl From<Status> for u64 {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => 0,
            Status::Accepted => 1,
            Status::WrongAnswer => 2,
            Status::CompileError => 3,
            Status::RuntimeError => 4,
            Status::TimeLimitExceed => 5,
            Status::MemoryLimitExceed => 6,
            Status::OutputLimitExceed => 7,
            Status::PresentationError => 8,
            Status::SystemError => 9,
            Status::UnkownError => 10,
        }
    }
}
