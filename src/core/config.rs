use std::fmt::Display;

/// Sanbox core config
#[derive(Debug)]
pub struct Config {
    pub code_type: Langs,
    pub bin_path: String,
    pub input_path: String,
    pub output_path: String,
    pub answer_path: String,
    pub error_path: String,

    pub real_time_limit: u64,
    pub cpu_time_limit: u64,
    pub max_memory: u64,
    pub max_stack: u64,
    pub max_process_number: u64,
    pub max_output_size: u64,

    pub arg: String,
    pub env: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            code_type: Langs::GENERAL,
            bin_path: String::new(),
            input_path: String::new(),
            output_path: String::new(),
            answer_path: String::new(),
            error_path: String::new(),
            real_time_limit: 8000,
            cpu_time_limit: 5000,
            max_memory: 0,
            max_stack: 0,
            max_process_number: 0,
            max_output_size: 0,
            arg: String::new(),
            env: String::new(),
        }
    }
}

/// Define Languages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Langs {
    GENERAL = 0,
    C = 1,
    CPP = 2,
    GOLANG = 3,
}

impl Display for Langs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Langs::GENERAL => write!(f, "GENERAL"),
            Langs::C => write!(f, "C"),
            Langs::CPP => write!(f, "CPP"),
            Langs::GOLANG => write!(f, "GOLANG"),
        }
    }
}
impl From<u64> for Langs {
    fn from(value: u64) -> Self {
        match value {
            0 => Langs::GENERAL,
            1 => Langs::C,
            2 => Langs::CPP,
            3 => Langs::GENERAL,
            _ => Langs::GENERAL,
        }
    }
}
