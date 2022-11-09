pub struct Rules {}

impl super::SeccompCtxRules for Rules {
    fn get_white_list(&self) -> Vec<&'static str> {
        let white_list = vec![
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
        ];

        white_list
    }
}
