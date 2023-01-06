use crate::core::config::{Config, Langs};
pub struct Cpp {}

impl Cpp {
    pub fn get_cmpile_config(base_dir: &std::path::PathBuf) -> Config {
        let input_path = base_dir.join("main.cpp");
        let output_path = base_dir.join("main");

        Config {
            code_type: Langs::CPP,
            bin_path: String::from("/usr/bin/g++"),
            input_path: input_path.clone().to_str().unwrap().to_string(),
            output_path: output_path.clone().to_str().unwrap().to_string(),
            error_path: String::new(),
            real_time_limit: 3000,
            cpu_time_limit: 3000,
            max_memory: 128,
            max_stack: 16,
            max_process_number: 0,
            max_output_size: 0,
            arg: String::from(format!(
                "/usr/bin/g++ -g -o {} {}",
                output_path.clone().to_str().unwrap(),
                input_path.clone().to_str().unwrap(),
            )),
            env: String::from("PATH=/usr/local/bin:/usr/bin:/bin"),
        }
    }
}
