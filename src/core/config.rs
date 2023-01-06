#[derive(Debug)]
pub struct Config {
    pub code_type: Langs,
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

    pub arg: String,
    pub env: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            code_type: Langs::GENERAL,
            bin_path: String::new(),
            input_path: String::new(),
            output_path: String::new(),
            error_path: String::new(),
            real_time_limit: 0,
            cpu_time_limit: 0,
            max_memory: 0,
            max_stack: 0,
            max_process_number: 0,
            max_output_size: 0,
            arg: String::new(),
            env: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Langs {
    GENERAL = 0,
    C = 1,
    CPP = 2,
    GOLANG = 3,
}
