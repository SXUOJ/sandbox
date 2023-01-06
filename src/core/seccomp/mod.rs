mod cpp;

use crate::{config::Langs, Result};
use libseccomp::{ScmpAction, ScmpFilterContext, ScmpSyscall};

pub fn load_seccomp_rules(code_type: Langs) -> Result<()> {
    match code_type {
        Langs::CPP => cpp::C_SECCOMP_RULES.load()?,
        _ => SeccompFilterConfig::default().load()?,
    };
    Ok(())
}

#[derive(Debug)]
pub struct SeccompFilterConfig<'a> {
    action: ScmpAction,
    allow_syscall: Vec<&'a str>,
    ban_syscall: Vec<&'a str>,
    arch_allow_syscall: Vec<&'a str>,
    arch_ban_syscall: Vec<&'a str>,
}

impl Default for SeccompFilterConfig<'_> {
    fn default() -> Self {
        Self {
            action: ScmpAction::Allow,
            allow_syscall: vec![],
            ban_syscall: vec![],
            arch_allow_syscall: vec![],
            arch_ban_syscall: vec![],
        }
    }
}

impl SeccompFilterConfig<'_> {
    fn load(&self) -> Result<()> {
        let mut filter = ScmpFilterContext::new_filter(self.action)?;

        for rule in self.allow_syscall.clone() {
            filter.add_rule(ScmpAction::Allow, ScmpSyscall::from_name(rule)?)?
        }

        // for rule in self.ban_syscall.clone() {
        // filter.add_rule(ScmpAction::KillProcess, ScmpSyscall::from_name(rule)?)?
        // }

        filter.load()?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::*;
    use nix::{
        libc,
        sys::wait::waitpid,
        unistd::{fork, write, ForkResult},
    };

    #[test]
    #[ignore]
    pub fn cpp_seccomp_loader() {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                println!(
                    "Continuing execution in parent process, new child has pid: {}",
                    child
                );
                waitpid(child, None).unwrap();
            }
            Ok(ForkResult::Child) => {
                write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
                load_seccomp_rules(Langs::CPP).unwrap();
                write(libc::STDOUT_FILENO, "Load seccomp rules\n".as_bytes()).ok();
                unsafe { libc::_exit(0) };
            }
            Err(_) => println!("Fork failed"),
        }
    }
}
