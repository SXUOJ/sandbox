use judge_core::config;

const WORK_SPACE: &str = "tmp";
const INPUT: &str = "input.txt";
const OUTPUT: &str = "output.txt";
const ANSWER: &str = "answer.txt";
const COMPILE_ERROR: &str = "cmpile_error.txt";
const RUN_ERROR: &str = "run_error.txt";

pub struct Config<'a> {
    pub code_type: Option<&'a str>,
    pub source: String,
    pub input: String,
    pub output: String,
    pub time_limit: u64,
    pub memory_limit: u64,
}

impl<'a> Config<'a> {
    pub fn init(&self) -> Result<(config::Config, config::Config), ()> {
        use crate::langs;
        use std::{fs::create_dir_all, path::Path};
        use uuid::Uuid;

        let base_dir = Path::new(WORK_SPACE).join(Uuid::new_v4().simple().to_string());
        create_dir_all(&base_dir).unwrap();
        // set compile config
        let mut compile_config = match self.code_type {
            Some("C") => langs::c::C::get_cmpile_config(&base_dir),
            Some("CPP") => langs::cpp::Cpp::get_cmpile_config(&base_dir),
            _ => config::Config::default(),
        };
        compile_config.error_path = base_dir.join(COMPILE_ERROR).to_str().unwrap().to_string();

        // save source
        self.save_source(&compile_config.input_path);

        // save input and output
        self.save_txt(&base_dir);

        // set run config
        let mut run_config = config::Config::default();
        run_config.bin_path = compile_config.output_path.to_owned();
        run_config.input_path = base_dir.join(INPUT).to_str().unwrap().to_string();
        run_config.output_path = base_dir.join(OUTPUT).to_str().unwrap().to_string();
        run_config.error_path = base_dir.join(RUN_ERROR).to_str().unwrap().to_string();

        Ok((compile_config, run_config))
    }

    fn save_source(&self, filepath: &String) {
        use std::fs::File;
        use std::io::Write;

        File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filepath)
            .unwrap()
            .write(&self.source.as_bytes())
            .unwrap();
    }

    fn save_txt(&self, base_dir: &std::path::PathBuf) {
        use std::fs::File;
        use std::io::Write;

        File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(base_dir.join(INPUT))
            .unwrap()
            .write(&self.input.as_bytes())
            .unwrap();

        File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(base_dir.join(ANSWER))
            .unwrap()
            .write(&self.output.as_bytes())
            .unwrap();
    }
}
