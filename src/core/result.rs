#[derive(Debug)]
pub struct JudgeResult {
    pub status: Result,
    pub signal: i32,
    pub real_time: f64,
    pub cpu_time: f64,
    pub memory: f64,
    pub error: String,
}

impl Default for JudgeResult {
    fn default() -> Self {
        Self {
            status: Result::Success,
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
    use nix::libc::{SIGSEGV, SIGUSR1, WEXITSTATUS, WIFSIGNALED, WTERMSIG};
    let mut result = JudgeResult::default();

    result.real_time = raw_result.real_time_cost.as_millis() as f64;

    if WIFSIGNALED(raw_result.exit_status) {
        result.signal = WTERMSIG(raw_result.exit_status)
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Result {
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
}
