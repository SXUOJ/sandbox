#[derive(Debug)]
pub struct Config<'a> {
    pub bin_path: &'a str,
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub error_path: &'a str,

    pub real_time_limit: u32,
    pub cpu_time_limit: u32,
    pub max_memory: u32,
    pub max_stack: u32,
    pub max_process_number: u32,
    pub max_output_size: u32,
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config {
            bin_path: "",
            input_path: "",
            output_path: "",
            error_path: "",
            real_time_limit: 0,
            cpu_time_limit: 0,
            max_memory: 0,
            max_stack: 0,
            max_process_number: 0,
            max_output_size: 0,
        }
    }
}
