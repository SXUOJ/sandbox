#[derive(Debug)]
pub struct JudgeResult {
    pub status: Result,
    pub signal: i32,
    pub real_time: f64,
    pub cpu_time: f64,
    pub memory: f64,
}

impl Default for JudgeResult {
    fn default() -> Self {
        Self {
            status: Result::Success,
            signal: 0,
            real_time: 0.0,
            cpu_time: 0.0,
            memory: 0.0,
        }
    }
}

pub fn infer_result(
    config: &crate::config::Config,
    raw_result: &crate::runner::RawJudgeResult,
) -> JudgeResult {
    use nix::libc::{SIGSEGV, SIGUSR1, WEXITSTATUS, WTERMSIG};
    let mut result = JudgeResult::default();

    result.real_time = raw_result.real_time_cost.as_millis() as f64;
    result.signal = WTERMSIG(raw_result.exit_status);

    if result.signal == SIGUSR1 {
        result.status = Result::SystemError;
    } else {
        result.cpu_time = raw_result.resource_usage.ru_utime.tv_sec as f64 * 1000.0
            + raw_result.resource_usage.ru_utime.tv_usec as f64 / 1000.0;
        result.memory = raw_result.resource_usage.ru_maxrss as f64 / 1024.0;

        if WEXITSTATUS(raw_result.exit_status) != 0 {
            result.status = Result::RuntimeError;
        }

        if result.signal == SIGSEGV {
            if config.max_memory != 0 && result.memory > config.max_memory as f64 {
                result.status = Result::MemoryLimitExceed;
            } else {
                result.status = Result::RuntimeError;
            }
        } else {
            if result.signal != 0 {
                result.status = Result::RuntimeError;
            }

            if config.max_memory != 0 && result.memory > config.max_memory as f64 {
                result.status = Result::MemoryLimitExceed;
            }

            if config.real_time_limit != 0 && result.real_time > config.real_time_limit as f64 {
                result.status = Result::TimeLimitExceed;
            }

            if config.cpu_time_limit != 0 && result.cpu_time > config.cpu_time_limit as f64 {
                result.status = Result::TimeLimitExceed;
            }
        }
    };

    result
}

#[derive(Debug)]
pub enum Result {
    Success,
    Accepted,
    WrongAnswer,
    // CompileError,
    RuntimeError,
    TimeLimitExceed,
    MemoryLimitExceed,
    OutputLimitExceed,
    PresentationError,
    SystemError,
}
