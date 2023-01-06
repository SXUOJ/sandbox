use super::*;
use lazy_static::lazy_static;
use libseccomp::ScmpAction;

lazy_static! {
pub static ref C_SECCOMP_RULES: SeccompFilterConfig<'static> = SeccompFilterConfig {
    action: ScmpAction::KillProcess,
    allow_syscall: vec![
        // base
        "fstat",
        "uname",
        "read",
        "pread64",
        "write",
        "writev",
        "mmap",
        "munmap",
        "mprotect",
        "brk",
        "close",
        "exit_group",
        "execve",
        "open",
        "openat",
        "arch_prctl",
        "newfstatat",
        "rseq",
        "faccessat",
        "set_tid_address",
        "set_robust_list",
        "prlimit64",
    ],
    ban_syscall: vec![],
    arch_allow_syscall: vec![],
    arch_ban_syscall: vec![],
};
}
