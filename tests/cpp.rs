#[cfg(test)]
mod tests {
    use judger::config::Config;
    use judger::result::infer_result;
    use judger::runner::run;

    fn compile(example_name: &str) -> String {
        let bin_path = format!("./examples/bin/cpp/{}", example_name);
        let src_path = format!("./examples/src/cpp/{}.cpp", example_name);

        let mut compile_config = Config::default();
        compile_config.bin_path = "/usr/bin/g++".to_string();
        compile_config.arg = format!("/usr/bin/g++ -g -o {} {}", bin_path, src_path);

        let res = run(&compile_config).unwrap().unwrap();
        println!("{:?}", infer_result(&compile_config, &res));

        bin_path
    }

    #[test]
    fn test_compile() {
        compile("hello");
    }

    #[test]
    fn test_infinite_loop() {
        let bin_path = compile("infinite_loop");

        let runner_config = Config {
            code_type: String::from("CPP"),
            bin_path,
            input_path: String::new(),
            output_path: String::new(),
            error_path: String::new(),
            real_time_limit: 5000,
            cpu_time_limit: 3000,
            max_memory: 128 * 1024,
            max_stack: 16 * 1024,
            max_process_number: 1,
            max_output_size: 256 * 1024,
            arg: String::new(),
            env: String::new(),
        };

        let res = run(&runner_config).unwrap().unwrap();
        println!("{:?}", infer_result(&runner_config, &res));
    }
}
