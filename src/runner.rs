use nix::libc::{c_int, rusage, WEXITSTATUS, WTERMSIG};
use std::time::Duration;
#[derive(Debug)]
pub struct RawJudgeResult {
    pub exit_status: c_int,
    pub exit_signal: c_int,
    pub exit_code: c_int,
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
                exit_signal: WTERMSIG(status),
                exit_code: WEXITSTATUS(status),
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
    fn test_read_write() {
        compile(
            "./test_cases/bin/cpp/hello",
            "./test_cases/src/cpp/hello.cpp",
        );

        let runner_config = crate::config::Config {
            code_type: String::from("CPP"),
            bin_path: String::from("./test_cases/bin/cpp/hello"),
            input_path: String::from("./test_cases/src/cpp/hello0.in"),
            output_path: String::from("./test_cases/src/cpp/hello0.out"),
            error_path: String::from("./test_cases/src/cpp/hello0.err"),
            real_time_limit: 1000,
            cpu_time_limit: 1000,
            max_memory: 128 * 1024 * 1024,
            max_stack: 16 * 1024,
            max_process_number: 1,
            max_output_size: 256 * 1024,
        };

        let res = run(&runner_config).unwrap().unwrap();
        println!("{:?}", infer_result(&res));
    }

    #[test]
    fn test_infinite_loop() {
        compile(
            "./test_cases/bin/cpp/infinite_loop",
            "./test_cases/src/cpp/infinite_loop.cpp",
        );

        let runner_config = crate::config::Config {
            code_type: String::from("CPP"),
            bin_path: String::from("./test_cases/bin/cpp/infinite_loop"),
            input_path: String::from("./test_cases/src/cpp/hello0.in"),
            output_path: String::from("./test_cases/src/cpp/hello0.out"),
            error_path: String::from("./test_cases/src/cpp/hello0.err"),
            real_time_limit: 5000,
            cpu_time_limit: 3000,
            max_memory: 128 * 1024,
            max_stack: 16 * 1024,
            max_process_number: 1,
            max_output_size: 256 * 1024,
        };

        let res = run(&runner_config).unwrap().unwrap();
        println!("{:?}", infer_result(&res));
    }
}
