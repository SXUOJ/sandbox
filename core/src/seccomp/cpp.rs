use super::*;
use lazy_static::lazy_static;
use libseccomp::ScmpAction;

lazy_static! {
    pub static ref C_SECCOMP_RULES: SeccompFilterConfig<'static> = SeccompFilterConfig {
        action: ScmpAction::KillProcess,
        allow_syscall: vec![
            "mprotect",
            "uname",
            "arch_prctl",
            "brk",
            "access",
            "faccessat",
            "readlink",
            "sysinfo",
            "clock_gettime",
            "execve",
            "open",
            "openat",
            "close",
            "read",
            "pread64",
            "readv",
            "write",
            "pwrite64",
            "writev",
            "lseek",
            "fstat",
            "newfstatat",
            "mmap",
            "munmap",
            "exit",
            "exit_group",
            "rseq",
            "prlimit64",
            "set_tid_address",
            "set_robust_list",
        ],
        ban_syscall: vec![],
        arch_allow_syscall: vec![],
        arch_ban_syscall: vec![],
    };
}
