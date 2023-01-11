use crate::core::{
    config,
    result::{infer_result, JudgeResult, Status},
    runner::run,
};

use crate::langs;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

const WORK_SPACE: &str = "tmp";
const INPUT: &str = "input.txt";
const OUTPUT: &str = "output.txt";
const ANSWER: &str = "answer.txt";
const COMPILE_ERROR: &str = "cmpile_error.txt";
const RUN_ERROR: &str = "run_error.txt";

#[derive(Debug)]
pub struct Sample {
    pub input: String,
    pub output: String,
}

#[derive(Debug)]
pub struct Judger {
    pub submit_id: String,
    pub code_type: u64,
    pub source: String,
    pub time_limit: u64,
    pub memory_limit: u64,

    pub samples: Vec<Sample>,

    pub base_dir: PathBuf,
    pub compile_config: config::Config,
}

impl Judger {
    fn init(&mut self) {
        self.base_dir = Path::new(WORK_SPACE).join(self.submit_id.clone());
        create_dir_all(&self.base_dir).unwrap();

        self.compile_config = langs::get_compile_config(self.code_type.into(), &self.base_dir);
        self.compile_config.error_path = self
            .base_dir
            .join(COMPILE_ERROR)
            .to_str()
            .unwrap()
            .to_string();
        self.save_file(&self.compile_config.input_path.clone().into(), &self.source);
    }

    pub fn run(&mut self) -> Vec<JudgeResult> {
        self.init();

        let mut results: Vec<JudgeResult> = vec![];
        let mut compile_result = infer_result(
            &self.compile_config,
            &run(&self.compile_config).unwrap().unwrap(),
        );

        if compile_result.status != Status::Success {
            compile_result.status = Status::CompileError;
            results.push(compile_result);
            return results;
        }

        for i in 1..=self.samples.len() {
            let workspace = self.base_dir.join(format!("case{}", i));
            create_dir_all(&workspace).unwrap();

            // save input and answer
            self.save_file(&workspace.join(INPUT), &self.samples[i - 1].input);
            self.save_file(&workspace.join(ANSWER), &self.samples[i - 1].output);

            let mut run_config = config::Config::default();
            run_config.code_type = self.code_type.into();
            run_config.bin_path = self.compile_config.output_path.to_owned();
            run_config.input_path = workspace.join(INPUT).to_str().unwrap().to_string();
            run_config.output_path = workspace.join(OUTPUT).to_str().unwrap().to_string();
            run_config.error_path = workspace.join(RUN_ERROR).to_str().unwrap().to_string();

            results.push(infer_result(
                &run_config,
                &run(&run_config).unwrap().unwrap(),
            ));
        }

        results
    }

    fn save_file(&self, filepath: &PathBuf, content: &str) {
        File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filepath)
            .unwrap()
            .write(content.as_bytes())
            .unwrap();
    }
}
