use super::*;
use lazy_static::lazy_static;
use libseccomp::ScmpAction;

lazy_static! {
    pub static ref C_SECCOMP_RULES: SeccompFilterConfig<'static> = SeccompFilterConfig {
        default_action: ScmpAction::KillProcess,
        syscall_list_action: ScmpAction::Allow,
        syscall_list: vec![
            "read",
            "fstat",
            "mmap",
            "mprotect",
            "munmap",
            "uname",
            "arch_prctl",
            "brk",
            "access",
            "exit_group",
            "close",
            "readlink",
            "sysinfo",
            "write",
            "writev",
            "lseek",
            "clock_gettime",
            "pread64",
            "execve",
            "open",
            "openat",
        ],
    };
}
