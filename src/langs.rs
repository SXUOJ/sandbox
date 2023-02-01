use crate::core::config::{Config, Langs};

pub fn get_compile_config(lang: Langs, base_dir: &std::path::PathBuf) -> Config {
    match lang {
        Langs::C => get_c_cmpile_config(base_dir),
        Langs::CPP => get_cpp_cmpile_config(base_dir),
        _ => Config::default(),
    }
}

fn get_c_cmpile_config(base_dir: &std::path::PathBuf) -> Config {
    let input_path = base_dir.join("main.c");
    let output_path = base_dir.join("main");

    Config {
        code_type: Langs::GENERAL,
        bin_path: String::from("/usr/bin/gcc"),
        input_path: input_path.clone().to_str().unwrap().to_string(),
        output_path: output_path.clone().to_str().unwrap().to_string(),
        error_path: String::new(),
        real_time_limit: 5000,
        cpu_time_limit: 3000,
        max_memory: 128 * 1024 * 1024,
        max_stack: 0,
        max_process_number: 0,
        max_output_size: 0,
        arg: String::from(format!(
            "/usr/bin/gcc -O2 -w -fmax-errors=3 -std=c99 {} -lm -o {}",
            input_path.clone().to_str().unwrap(),
            output_path.clone().to_str().unwrap(),
        )),
        env: String::from("PATH=/usr/local/bin:/usr/bin:/bin"),
    }
}

fn get_cpp_cmpile_config(base_dir: &std::path::PathBuf) -> Config {
    let input_path = base_dir.join("main.cpp");
    let output_path = base_dir.join("main");

    Config {
        code_type: Langs::GENERAL,
        bin_path: String::from("/usr/bin/g++"),
        input_path: input_path.clone().to_str().unwrap().to_string(),
        output_path: output_path.clone().to_str().unwrap().to_string(),
        error_path: String::new(),
        real_time_limit: 5000,
        cpu_time_limit: 3000,
        max_memory: 128 * 1024 * 1024,
        max_stack: 0,
        max_process_number: 0,
        max_output_size: 0,
        arg: String::from(format!(
            "/usr/bin/g++ -O2 -w -fmax-errors=3 -std=c++11 {} -lm -o {}",
            input_path.clone().to_str().unwrap(),
            output_path.clone().to_str().unwrap(),
        )),
        env: String::from("PATH=/usr/local/bin:/usr/bin:/bin"),
    }
}
