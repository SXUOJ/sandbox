pub fn run(config: &crate::config::Config) -> Result<Option<()>, crate::error::CoreError> {
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

            Ok(None)
        }
        Ok(ForkResult::Child) => {
            crate::fork::after_fork(config)?;
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
pub mod runner {
    use super::*;
    fn compile() {
        use std::process::Command;

        Command::new("g++")
            .arg("-g")
            .arg("-o")
            .arg("./test_cases/bin/cpp/hello")
            .arg("./test_cases/src/cpp/hello.cpp")
            .output()
            .expect("Compile Error");
    }

    #[test]
    fn test_run() {
        compile();
        let runner_config = crate::config::Config {
            bin_path: String::from("./test_cases/bin/cpp/hello"),
            input_path: String::from("./test_cases/src/hello0.in"),
            output_path: String::from("./test_cases/src/hello0.out"),
            error_path: String::from(""),
            real_time_limit: 1000,
            cpu_time_limit: 1000,
            max_memory: 128 * 1024,
            max_stack: 16 * 1024,
            max_process_number: 1,
            max_output_size: 8 * 1024,
        };
        run(&runner_config);
    }
}
