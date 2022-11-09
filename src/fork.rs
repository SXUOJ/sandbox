pub fn after_fork(config: &crate::config::Config) -> Result<(), crate::error::CoreError> {
    use std::{
        ffi::CString,
        fs::File,
        io,
        os::unix::io::{AsRawFd, RawFd},
    };

    use nix::unistd::{dup2, execve};

    set_resource_limit(&ResourceLimitConfig {
        cpu_limit: Some((config.cpu_time_limit, config.cpu_time_limit)),
        as_limit: Some((config.max_memory, config.max_memory)),
        stack_limit: Some((config.max_stack, config.max_stack)),
        nproc_limit: Some((config.max_process_number, config.max_process_number)),
        fsize_limit: Some((config.max_output_size, config.max_output_size)),
    })
    .unwrap();

    let input_file = File::open(&config.input_path)?;
    let output_file = File::options()
        .write(true)
        .truncate(true)
        .open(&config.output_path)
        .unwrap();

    let input_raw_fd: RawFd = input_file.as_raw_fd();
    let stdin_raw_fd: RawFd = io::stdin().as_raw_fd();
    dup2(input_raw_fd, stdin_raw_fd)?;

    let output_raw_fd: RawFd = output_file.as_raw_fd();
    let stdout_raw_fd: RawFd = io::stdout().as_raw_fd();
    dup2(output_raw_fd, stdout_raw_fd)?;

    println!("Oh...");

    execve(
        &CString::new(config.bin_path.as_str())?,
        &[CString::new("")?],
        &[CString::new("")?],
    )
    .unwrap();

    Ok(())
}

#[derive(Default)]
struct ResourceLimitConfig {
    pub stack_limit: Option<(u64, u64)>,
    pub as_limit: Option<(u64, u64)>,
    pub cpu_limit: Option<(u64, u64)>,
    pub nproc_limit: Option<(u64, u64)>,
    pub fsize_limit: Option<(u64, u64)>,
}

fn set_resource_limit(config: &ResourceLimitConfig) -> Result<(), nix::errno::Errno> {
    use nix::sys::resource::{
        setrlimit,
        Resource::{RLIMIT_AS, RLIMIT_CPU, RLIMIT_FSIZE, RLIMIT_STACK},
    };

    if let Some(as_limit) = config.as_limit {
        setrlimit(RLIMIT_AS, as_limit.0, as_limit.1)?;
    }

    if let Some(cpu_limit) = config.cpu_limit {
        setrlimit(RLIMIT_CPU, cpu_limit.0, cpu_limit.1)?;
    }

    if let Some(fsize_limit) = config.fsize_limit {
        setrlimit(RLIMIT_FSIZE, fsize_limit.0, fsize_limit.1)?;
    }

    if let Some(nproc_limit) = config.nproc_limit {
        setrlimit(RLIMIT_NPROC, nproc_limit.0, nproc_limit.1)?;
    }

    if let Some(stack_limit) = config.stack_limit {
        setrlimit(RLIMIT_STACK, stack_limit.0, stack_limit.1)?;
    }

    Ok(())
}
