pub fn child_process(config: &crate::config::Config) -> Result<(), crate::error::CoreError> {
    // set  rlimit
    #[cfg(target_os = "linux")]
    set_rlimit(&config)?;

    // load input, output, error file path
    use std::{
        ffi::CString,
        fs::File,
        io,
        os::unix::io::{AsRawFd, RawFd},
    };

    if !config.input_path.is_empty() {
        let input_file = File::open(&config.input_path)?;
        let input_raw_fd: RawFd = input_file.as_raw_fd();
        let stdin_raw_fd: RawFd = io::stdin().as_raw_fd();
        nix::unistd::dup2(input_raw_fd, stdin_raw_fd)?;
    }

    if !config.output_path.is_empty() {
        let output_file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config.output_path.as_str())
            .unwrap();
        let output_raw_fd: RawFd = output_file.as_raw_fd();
        let stdout_raw_fd: RawFd = io::stdout().as_raw_fd();
        nix::unistd::dup2(output_raw_fd, stdout_raw_fd)?;
    }

    if !config.error_path.is_empty() {
        let error_file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&config.error_path)
            .unwrap();

        let error_raw_fd: RawFd = error_file.as_raw_fd();
        let stderr_raw_fd: RawFd = io::stderr().as_raw_fd();
        nix::unistd::dup2(error_raw_fd, stderr_raw_fd)?;
    }

    // nix::unistd::setgid(nix::unistd::Gid::from(20))?;
    // nix::unistd::setuid(nix::unistd::Uid::from(501))?;

    // load seccomp rules
    #[cfg(target_os = "linux")]
    crate::seccomp::load_rules_by_code_type(Some(&config.code_type)).unwrap();

    let arg = config
        .arg
        .to_string()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect::<Vec<CString>>();

    let env = config
        .env
        .to_string()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|env| CString::new(env.as_str()).unwrap())
        .collect::<Vec<CString>>();

    // exec
    nix::unistd::execve(&CString::new(config.bin_path.as_str())?, &arg, &env).unwrap();

    Ok(())
}

#[cfg(target_os = "linux")]
fn set_rlimit(config: &crate::config::Config) -> Result<(), nix::errno::Errno> {
    use nix::sys::resource::{setrlimit, Resource};

    if config.cpu_time_limit != 0 {
        setrlimit(
            Resource::RLIMIT_CPU,
            config.cpu_time_limit / 1000,
            config.cpu_time_limit / 1000,
        )?;
    }

    if config.max_memory != 0 {
        setrlimit(Resource::RLIMIT_AS, config.max_memory, config.max_memory)?;
    }

    if config.max_stack != 0 {
        setrlimit(Resource::RLIMIT_STACK, config.max_stack, config.max_stack)?;
    }

    if config.max_process_number != 0 {
        setrlimit(
            Resource::RLIMIT_NPROC,
            config.max_process_number,
            config.max_process_number,
        )?;
    }

    if config.max_output_size != 0 {
        setrlimit(
            Resource::RLIMIT_FSIZE,
            config.max_output_size,
            config.max_output_size,
        )?;
    }

    Ok(())
}
