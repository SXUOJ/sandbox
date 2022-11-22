#[cfg(test)]
mod tests {
    use sandbox::config::Config;
    use sandbox::result::infer_result;
    use sandbox::runner::run;

    fn compile(example_name: &str) -> String {
        let bin_path = format!("./examples/bin/cpp/{}", example_name);
        let src_path = format!("./examples/src/cpp/{}.cpp", example_name);
        let error_path = format!("./examples/src/cpp/{}-compile.err", example_name);

        let mut compile_config = Config::default();
        compile_config.bin_path = "/usr/bin/g++".to_string();
        compile_config.arg = format!("/usr/bin/g++ -g -o {} {}", bin_path, src_path);
        compile_config.env = format!("PATH=/usr/local/bin:/usr/bin:/bin");
        compile_config.error_path = error_path;

        let res = run(&compile_config).unwrap().unwrap();
        let result = infer_result(&compile_config, &res);
        match result.status {
            sandbox::result::Result::Success => println!("Compile Success!"),
            _ => {
                println!("[COMPILE]:{}: {:?}", example_name, result);
                panic!("Compile error!");
            }
        };

        bin_path
    }

    fn run_just_example_name(example_name: &str) {
        let bin_path = compile(example_name);

        let mut runner_config = Config::default();
        runner_config.bin_path = bin_path;
        runner_config.code_type = "CPP".to_string();

        let res = run(&runner_config).unwrap().unwrap();
        println!(
            "[RUN]:{}: {:?}",
            example_name,
            infer_result(&runner_config, &res)
        );
    }

    #[test]
    fn test_compile_hello() {
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

    #[test]
    fn test_plain_text() {
        run_just_example_name("plain_text")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_1() {
        compile("compiler_bomb_1");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_2() {
        compile("compiler_bomb_2");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_3() {
        compile("compiler_bomb_3");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_4() {
        compile("compiler_bomb_4");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_core_dump_0() {
        run_just_example_name("core_dump_0")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_fork_bomb() {
        run_just_example_name("fork_bomb")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_include_leaks() {
        run_just_example_name("include_leaks")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_memory_allocation() {
        run_just_example_name("memory_allocation")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_run_command_line_0() {
        run_just_example_name("run_command_line_0")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_thread() {
        run_just_example_name("thread")
    }
}
