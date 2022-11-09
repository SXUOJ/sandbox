#[derive(Debug)]
pub struct JudgeResult {
    pub exit_status: u64,
    pub exit_signal: u64,
    pub exit_code: u64,
    pub real_time: f64,
    pub cpu_time: f64,
    pub memory: f64,
}

pub fn infer_result(raw_result: &crate::runner::RawJudgeResult) -> JudgeResult {
    println!("{:?}", raw_result.resource_usage);
    JudgeResult {
        exit_status: raw_result.exit_status as u64,
        exit_signal: raw_result.exit_signal as u64,
        exit_code: raw_result.exit_code as u64,
        real_time: raw_result.real_time_cost.as_millis() as f64,
        cpu_time: raw_result.resource_usage.ru_utime.tv_sec as f64 * 1000.0
            + raw_result.resource_usage.ru_utime.tv_usec as f64 / 1000.0,
        memory: raw_result.resource_usage.ru_maxrss as f64 / 1024.0,
    }
}
