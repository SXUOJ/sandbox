mod cpp;

use crate::core::{config::Langs, Result};
use libseccomp::{ScmpAction, ScmpArgCompare, ScmpCompareOp, ScmpFilterContext, ScmpSyscall};
pub fn load_seccomp_rules(code_type: Langs) -> Result<()> {
    match code_type {
        Langs::CPP => cpp::C_SECCOMP_RULES.load()?,
        _ => SeccompFilterConfig::default().load()?,
    };
    Ok(())
}

#[derive(Debug)]
pub struct SeccompFilterConfig<'a> {
    default_action: ScmpAction,
    syscall_list: Vec<&'a str>,
    syscall_list_action: ScmpAction,
}

impl Default for SeccompFilterConfig<'_> {
    fn default() -> Self {
        Self {
            default_action: ScmpAction::Allow,
            syscall_list: vec![],
            syscall_list_action: ScmpAction::KillProcess,
        }
    }
}

impl SeccompFilterConfig<'_> {
    fn load(&self) -> Result<()> {
        let mut filter = ScmpFilterContext::new_filter(self.default_action)?;
        for rule in self.syscall_list.clone() {
            filter.add_rule(self.syscall_list_action, ScmpSyscall::from_name(rule)?)?
        }

        filter.load()?;
        Ok(())
    }
}
