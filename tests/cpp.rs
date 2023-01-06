#[cfg(test)]
mod tests {
    use sandbox::core::{
        config::{Config, Langs},
        result::{infer_result, JudgeResult, Result},
        runner,
    };

    fn compile(example_name: &str) -> (String, JudgeResult) {
        let bin_path = format!("./examples/bin/cpp/{}", example_name);
        let src_path = format!("./examples/src/cpp/{}.cpp", example_name);
        let error_path = format!("./examples/src/cpp/{}-compile.err", example_name);

        let mut compile_config = Config::default();
        compile_config.bin_path = "/usr/bin/g++".to_string();
        compile_config.arg = format!("/usr/bin/g++ -g -o {} {}", bin_path, src_path);
        compile_config.env = format!("PATH=/usr/local/bin:/usr/bin:/bin");
        compile_config.error_path = error_path;

        (
            bin_path,
            infer_result(
                &compile_config,
                &runner::run(&compile_config).unwrap().unwrap(),
            ),
        )
    }

    fn compile_and_get_run_config(example_name: &str) -> Config {
        let (bin_path, compile_result) = compile(example_name);
        assert_eq!(
            compile_result.status,
            Result::Success,
            "[CE]: `{}`",
            compile_result
        );

        let mut runner_config = Config::default();
        runner_config.bin_path = bin_path;
        runner_config.code_type = Langs::CPP;

        runner_config
    }

    fn run(config: &Config) -> JudgeResult {
        infer_result(&config, &runner::run(&config).unwrap().unwrap())
    }

    #[test]
    fn test_compile_hello() {
        let (_, compile_config) = compile("hello");
        assert_eq!(
            compile_config.status,
            Result::Success,
            "[CE]: `{}",
            compile_config
        )
    }

    #[test]
    fn test_infinite_loop() {
        let mut runner_config = compile_and_get_run_config("infinite_loop");

        runner_config.real_time_limit = 2000;
        runner_config.cpu_time_limit = 2000;
        runner_config.max_memory = 128 * 1024 * 1024;
        runner_config.max_stack = 16 * 1024;
        runner_config.max_process_number = 1;
        runner_config.max_output_size = 256 * 1024;

        let result = run(&runner_config);
        assert_eq!(result.status, Result::TimeLimitExceed, "{}", result);
    }

    #[test]
    fn test_plain_text() {
        let (_, compile_result) = compile("plain_text");
        assert_eq!(
            compile_result.status,
            Result::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_1() {
        let (_, compile_result) = compile("compiler_bomb_1");
        assert_eq!(
            compile_result.status,
            Result::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_2() {
        let (_, compile_result) = compile("compiler_bomb_2");
        assert_eq!(
            compile_result.status,
            Result::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_3() {
        let (_, compile_result) = compile("compiler_bomb_3");
        assert_eq!(
            compile_result.status,
            Result::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_compiler_bomb_4() {
        let (_, compile_result) = compile("compiler_bomb_4");
        assert_eq!(
            compile_result.status,
            Result::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }
}
