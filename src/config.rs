#[derive(Debug, Default)]
pub struct Config {
    pub bin_path: String,
    pub input_path: String,
    pub output_path: String,
    pub error_path: String,

    pub real_time_limit: u64,
    pub cpu_time_limit: u64,
    pub max_memory: u64,
    pub max_stack: u64,
    pub max_process_number: u64,
    pub max_output_size: u64,
}
