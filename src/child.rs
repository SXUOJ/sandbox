pub fn child_process(config: &crate::config::Config) -> Result<(), crate::error::CoreError> {
    // set  rlimit
    are_you_on_linux(&config)?;

    // load input, output, error file path
    use std::{
        ffi::CString,
        fs::File,
        io,
        os::unix::io::{AsRawFd, RawFd},
    };

    let input_file = File::open(&config.input_path)?;
    let output_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(config.output_path.as_str())
        .unwrap();

    let error_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config.error_path)
        .unwrap();

    let input_raw_fd: RawFd = input_file.as_raw_fd();
    let stdin_raw_fd: RawFd = io::stdin().as_raw_fd();
    nix::unistd::dup2(input_raw_fd, stdin_raw_fd)?;

    let output_raw_fd: RawFd = output_file.as_raw_fd();
    let stdout_raw_fd: RawFd = io::stdout().as_raw_fd();
    nix::unistd::dup2(output_raw_fd, stdout_raw_fd)?;

    let error_raw_fd: RawFd = error_file.as_raw_fd();
    let stderr_raw_fd: RawFd = io::stderr().as_raw_fd();
    nix::unistd::dup2(error_raw_fd, stderr_raw_fd)?;

    nix::unistd::setgid(nix::unistd::Gid::from(20))?;
    nix::unistd::setuid(nix::unistd::Uid::from(501))?;

    // load seccomp rules
    #[cfg(target_os = "linux")]
    crate::seccomp::load_rules_by_code_type(Some(&config.code_type)).unwrap();

    // exec
    nix::unistd::execve(
        &CString::new(config.bin_path.as_str())?,
        &[CString::new("")?],
        &[CString::new("")?],
    )
    .unwrap();

    Ok(())
}

#[cfg(target_os = "macos")]
fn are_you_on_linux(config: &crate::config::Config) -> Result<(), nix::errno::Errno> {
    use nix::sys::resource::{setrlimit, Resource};
    setrlimit(
        Resource::RLIMIT_CPU,
        config.cpu_time_limit / 1000,
        config.cpu_time_limit / 1000,
    )?;

    setrlimit(Resource::RLIMIT_AS, config.max_memory, config.max_memory)?;
    setrlimit(Resource::RLIMIT_STACK, config.max_stack, config.max_stack)?;

    setrlimit(
        Resource::RLIMIT_FSIZE,
        config.max_output_size,
        config.max_output_size,
    )?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn are_you_on_linux(config: &crate::config::Config) -> Result<(), nix::errno::Errno> {
    use nix::sys::resource::{setrlimit, Resource};
    setrlimit(
        Resource::RLIMIT_CPU,
        config.cpu_time_limit / 1000,
        config.cpu_time_limit / 1000,
    )?;

    setrlimit(Resource::RLIMIT_AS, config.max_memory, config.max_memory)?;
    setrlimit(Resource::RLIMIT_STACK, config.max_stack, config.max_stack)?;

    setrlimit(
        Resource::RLIMIT_NPROC,
        config.max_process_number,
        config.max_process_number,
    )?;
    setrlimit(
        Resource::RLIMIT_FSIZE,
        config.max_output_size,
        config.max_output_size,
    )?;

    Ok(())
}
