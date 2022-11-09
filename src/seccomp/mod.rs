mod c;
mod cpp;
mod general;
mod golang;

use libseccomp::{error::SeccompError, ScmpAction, ScmpFilterContext, ScmpSyscall};

pub fn load_rules_by_code_type(code_type: Option<&str>) -> Result<(), SeccompError> {
    match code_type {
        Some("C") => load_write_list(Box::new(c::Rules {})),
        Some("CPP") => load_write_list(Box::new(cpp::Rules {})),
        Some("Golang") => load_write_list(Box::new(golang::Rules {})),
        _ => load_write_list(Box::new(general::Rules {})),
    }
}

fn load_write_list(ctx_rules: Box<dyn SeccompCtxRules>) -> Result<(), SeccompError> {
    let mut ctx = get_default_kill_context().unwrap();
    for syscall_name in ctx_rules.get_white_list() {
        ctx.add_rule_exact(ScmpAction::Allow, ScmpSyscall::from_name(syscall_name)?)?;
    }

    ctx.load()?;
    Ok(())
}

fn load_blach_list(ctx_rules: Box<dyn SeccompCtxRules>) -> Result<(), SeccompError> {
    let mut ctx = get_default_kill_context().unwrap();
    for syscall_name in ctx_rules.get_black_list() {
        ctx.add_rule_exact(
            ScmpAction::KillProcess,
            ScmpSyscall::from_name(syscall_name)?,
        )?;
    }

    ctx.load()?;
    Ok(())
}

fn get_default_kill_context() -> Result<ScmpFilterContext, SeccompError> {
    ScmpFilterContext::new_filter(ScmpAction::KillProcess)
}

trait SeccompCtxRules {
    fn get_white_list(&self) -> Vec<&'static str> {
        vec![]
    }
    fn get_black_list(&self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use nix::{
        sys::wait::waitpid,
        unistd::{fork, ForkResult},
    };

    #[test]
    pub fn test_cpp_loader() {
        use super::*;

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                waitpid(child, None).unwrap();
            }
            Ok(ForkResult::Child) => {
                load_rules_by_code_type(Some("CPP"));
                unsafe { nix::libc::_exit(0) };
            }
            Err(_) => println!("Fork failed"),
        }
    }
}
