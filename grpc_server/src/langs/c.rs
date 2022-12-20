use judge_core::config::{Config, Langs};
pub struct C {}

impl C {
    pub fn get_cmpile_config(base_dir: &std::path::PathBuf) -> Config {
        let input_path = base_dir.join("main.c");
        let output_path = base_dir.join("main");
        let error_path = base_dir.join("compile.err").to_str().unwrap().to_string();

        Config {
            code_type: Langs::C,
            bin_path: String::from("/usr/bin/gcc"),
            input_path: input_path.clone().to_str().unwrap().to_string(),
            output_path: output_path.clone().to_str().unwrap().to_string(),
            error_path,
            real_time_limit: 3000,
            cpu_time_limit: 3000,
            max_memory: 0,
            max_stack: 0,
            max_process_number: 0,
            max_output_size: 0,
            arg: String::from(format!(
                "/usr/bin/gcc -g -o {} {}",
                output_path.clone().to_str().unwrap(),
                input_path.clone().to_str().unwrap(),
            )),
            env: String::from("PATH=/usr/local/bin:/usr/bin:/bin"),
        }
    }
}
