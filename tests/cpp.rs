// #[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use sandbox::core::{
        config::{Config, Langs},
        result::{infer_result, JudgeResult, Status},
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
            Status::Success,
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
            Status::Success,
            "[CE]: `{}",
            compile_config
        )
    }

    #[test]
    fn test_compiler_text() {
        let (_, compile_result) = compile("text");
        assert_eq!(
            compile_result.status,
            Status::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[test]
    fn test_compile_error_0() {
        let (_, compile_result) = compile("compile_error_0");
        assert_eq!(
            compile_result.status,
            Status::TimeLimitExceed,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[test]
    fn test_compile_error_1() {
        let (_, compile_result) = compile("compile_error_1");
        assert_eq!(
            compile_result.status,
            Status::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[test]
    fn test_compile_error_2() {
        let (_, compile_result) = compile("compile_error_2");
        assert_eq!(
            compile_result.status,
            Status::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[test]
    fn test_compile_error_3() {
        let (_, compile_result) = compile("compile_error_3");
        assert_eq!(
            compile_result.status,
            Status::TimeLimitExceed,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[test]
    fn test_infinite_loop() {
        let runner_config = compile_and_get_run_config("infinite_loop");
        let result = run(&runner_config);
        assert_eq!(result.status, Status::TimeLimitExceed, "{}", result);
    }

    #[test]
    fn test_hello() {
        let runner_config = compile_and_get_run_config("hello");
        let result = run(&runner_config);
        assert_eq!(result.status, Status::Success, "{}", result);
    }

    #[test]
    fn test_read_write() {
        let mut runner_config = compile_and_get_run_config("read_write");
        runner_config.input_path = "./examples/src/cpp/read_write.in".to_string();
        runner_config.output_path = "./examples/src/cpp/read_write.out".to_string();
        let result = run(&runner_config);
        assert_eq!(result.status, Status::Success, "{}", result);
    }

    #[test]
    fn test_core_dump() {
        let runner_config = compile_and_get_run_config("core_dump");
        let result = run(&runner_config);
        assert_eq!(result.status, Status::RuntimeError, "{}", result);
    }

    #[test]
    fn test_fork() {
        let runner_config = compile_and_get_run_config("fork");
        let result = run(&runner_config);
        assert_eq!(result.status, Status::RuntimeError, "{}", result);
    }

    #[test]
    fn test_leaks() {
        let (_, compile_result) = compile("leaks");
        assert_eq!(
            compile_result.status,
            Status::RuntimeError,
            "[CE]: `{}`",
            compile_result
        )
    }

    #[test]
    fn test_memory_alloc() {
        let mut runner_config = compile_and_get_run_config("memory_alloc");
        runner_config.max_memory = 128 * 1024 * 1024;
        let result = run(&runner_config);
        assert_eq!(result.status, Status::RuntimeError, "{}", result);
    }

    #[test]
    fn test_reboot() {
        let runner_config = compile_and_get_run_config("reboot");
        let result = run(&runner_config);
        assert_eq!(result.status, Status::RuntimeError, "{}", result);
    }
}
