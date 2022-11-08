#[derive(Debug, Default)]
pub struct Config {
    pub bin_path: String,
    pub input_path: String,
    pub output_path: String,
    pub error_path: String,

    pub real_time_limit: u32,
    pub cpu_time_limit: u32,
    pub max_memory: u32,
    pub max_stack: u32,
    pub max_process_number: u32,
    pub max_output_size: u32,
}
