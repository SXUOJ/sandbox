use judger::config::Config;
use judger::result::infer_result;
use judger::runner::run;

#[warn(dead_code)]
use clap::{arg, value_parser, Command};

fn main() {
    let cmd = Command::new("Judger")
        .author("ther")
        .about("A judger for SXU Online Judge")
        .version("0.0.1")
        .arg_required_else_help(true)
        .arg(arg!(-t --code_type  <CODE_TYPE> "Code type.").value_parser(value_parser!(String)))
        .arg(arg!(-b --bin_path  <BIN_PATH> "Bin Path.").value_parser(value_parser!(String)))
        .arg(arg!(-i --input_path  <INPUT_PATH> "Input Path.").value_parser(value_parser!(String)))
        .arg(
            arg!(-o --output_path  <OUTPUT_PATH> "Output Path.")
                .value_parser(value_parser!(String)),
        )
        .arg(arg!(-e --error_path  <ERROR_PATH> "Error Path.").value_parser(value_parser!(String)))
        .arg(
            arg!(-r --real_time_limit  <REAL_TIME_LIMIT> "Real time limit.")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!(-c --cpu_time_limit <CPU_TIME_LIMIT> "CPU time limit.")
                .value_parser(value_parser!(u64)),
        )
        .arg(arg!(-m --max_memory <MAX_MEMORY> "Max memory.").value_parser(value_parser!(u64)))
        .arg(arg!(-s --max_stack <MAX_STACK> "Max stack.").value_parser(value_parser!(u64)))
        .arg(
            arg!(-p --max_process_number <max_process_number> "Max process number.")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!(-z --max_output_size <MAX_OUTPUT_SIZE> "Max output size.")
                .value_parser(value_parser!(u64)),
        )
        .arg(arg!(--arg <ARG> "Args.").value_parser(value_parser!(String)))
        .arg(arg!(--env <ENV> "Envs.").value_parser(value_parser!(String)))
        .get_matches();

    let config = parse_config(&cmd);
    println!("{:?}", config);

    let raw_judge_result = run(&config).unwrap().unwrap();
    println!("{:?}", infer_result(&raw_judge_result));
}

fn parse_config(matches: &clap::ArgMatches) -> Config {
    let mut config = Config::default();

    if matches.contains_id("code_type") {
        config.code_type = matches.get_one::<String>("code_type").unwrap().to_string();
    }

    if matches.contains_id("bin_path") {
        config.bin_path = matches.get_one::<String>("bin_path").unwrap().to_string();
    }

    if matches.contains_id("input_path") {
        config.input_path = matches.get_one::<String>("input_path").unwrap().to_string();
    }

    if matches.contains_id("output_path") {
        config.output_path = matches
            .get_one::<String>("output_path")
            .unwrap()
            .to_string();
    }

    if matches.contains_id("error_path") {
        config.error_path = matches.get_one::<String>("error_path").unwrap().to_string();
    }

    if matches.contains_id("real_time_limit") {
        config.real_time_limit = *matches.get_one::<u64>("real_time_limit").unwrap();
    }

    if matches.contains_id("cpu_time_limit") {
        config.cpu_time_limit = *matches.get_one::<u64>("cpu_time_limit").unwrap();
    }

    if matches.contains_id("max_memory") {
        config.max_memory = *matches.get_one::<u64>("max_memory").unwrap();
    }

    if matches.contains_id("max_stack") {
        config.max_stack = *matches.get_one::<u64>("max_stack").unwrap();
    }

    if matches.contains_id("max_process_number") {
        config.max_process_number = *matches.get_one::<u64>("max_process_number").unwrap();
    }

    if matches.contains_id("max_output_size") {
        config.max_output_size = *matches.get_one::<u64>("max_output_size").unwrap();
    }

    if matches.contains_id("arg") {
        config.arg = matches.get_one::<String>("arg").unwrap().to_string();
    }

    if matches.contains_id("env") {
        config.env = matches.get_one::<String>("env").unwrap().to_string();
    }

    config
}
