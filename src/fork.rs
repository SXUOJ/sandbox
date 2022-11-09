pub fn after_fork(config: &crate::config::Config) -> Result<(), crate::error::CoreError> {
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

    nix::unistd::execve(
        &CString::new(config.bin_path.as_str())?,
        &[CString::new("")?],
        &[CString::new("")?],
    )
    .unwrap();

    Ok(())
}
