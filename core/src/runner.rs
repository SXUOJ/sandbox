use nix::libc::{c_int, rusage};
use std::time::Duration;
#[derive(Debug)]
pub struct RawJudgeResult {
    pub exit_status: c_int,
    pub real_time_cost: Duration,
    pub resource_usage: rusage,
}

pub fn run(config: &crate::config::Config) -> crate::Result<Option<RawJudgeResult>> {
    use nix::unistd::{fork, ForkResult};
    use std::time::Instant;

    let start = Instant::now();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            use crate::killer::timeout_killer;
            use nix::libc::{wait4, WSTOPPED};
            use std::thread;

            match config.real_time_limit {
                timeout => {
                    if timeout != 0 {
                        thread::spawn(move || timeout_killer(child.as_raw() as u32, timeout + 500));
                    }
                }
            }

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
