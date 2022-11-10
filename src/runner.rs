use nix::libc::{c_int, rusage};
use std::time::Duration;
#[derive(Debug)]
pub struct RawJudgeResult {
    pub exit_status: c_int,
    pub real_time_cost: Duration,
    pub resource_usage: rusage,
}

pub fn run(
    config: &crate::config::Config,
) -> Result<Option<RawJudgeResult>, crate::error::CoreError> {
    use nix::unistd::{fork, ForkResult};
    use std::time::Instant;

    let start = Instant::now();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            use crate::killer::timeout_killer;
            use nix::libc::{wait4, WSTOPPED};
            use std::thread;

            let timeout = config.real_time_limit;
            thread::spawn(move || timeout_killer(child.as_raw() as u32, timeout));

            let mut status: nix::libc::c_int = 0;
            let mut usage = default_rusage();
            unsafe {
                wait4(child.as_raw() as i32, &mut status, WSTOPPED, &mut usage);
            }

            Ok(Some(RawJudgeResult {
                exit_status: status,
                real_time_cost: start.elapsed(),
                resource_usage: usage,
            }))
        }
        Ok(ForkResult::Child) => {
            crate::child::child_process(config)?;
            Ok(None)
        }
        Err(_) => {
            panic!("Fork failed!");
        }
    }
}

fn default_rusage() -> nix::libc::rusage {
    nix::libc::rusage {
        ru_utime: nix::libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: nix::libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::infer_result;

    fn compile(bin_name: &str, src_name: &str) {
        use std::process::Command;

        Command::new("g++")
            .arg("-g")
            .arg("-o")
            .arg(bin_name)
            .arg(src_name)
            .output()
            .expect("Compile Error");
    }

    #[test]
    #[ignore]
    fn test_run() {
        let bin_path = "./examples/bin/cpp/hello";
        let mut runner_config = crate::config::Config::default();
        runner_config.bin_path = bin_path.to_string();

        compile(bin_path, "./examples/src/cpp/hello.cpp");

        let res = run(&runner_config).unwrap().unwrap();
        println!("{:?}", infer_result(&runner_config, &res));
    }

    #[test]
    #[ignore]
    fn test_read_write() {
        compile(
            "./examples/bin/cpp/read_write",
            "./examples/src/cpp/read_write.cpp",
        );

        let runner_config = crate::config::Config {
            code_type: String::from("CPP"),
            bin_path: String::from("./examples/bin/cpp/read_write"),
            input_path: String::from("./examples/src/cpp/read_write.in"),
            output_path: String::from("./examples/src/cpp/read_write.out"),
            error_path: String::from("./examples/src/cpp/read_write.err"),
            real_time_limit: 1000,
            cpu_time_limit: 1000,
            max_memory: 128 * 1024 * 1024,
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
    #[ignore]
    fn test_infinite_loop() {
        compile(
            "./examples/bin/cpp/infinite_loop",
            "./examples/src/cpp/infinite_loop.cpp",
        );

        let runner_config = crate::config::Config {
            code_type: String::from("CPP"),
            bin_path: String::from("./examples/bin/cpp/infinite_loop"),
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
