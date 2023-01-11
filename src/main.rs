use clap::{arg, command, value_parser, Command};
use tonic::transport::Server;

use sandbox::{
    core::{
        config::{Config, Langs},
        result, runner,
    },
    grpc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("grpc")
                .about("Run a grpc server")
                .arg(arg!([ADDR])),
        )
        .subcommand(
            Command::new("cmd")
                .about("Command line tool")
                .arg_required_else_help(true)
                .arg(
                    arg!(-t --code_type  <CODE_TYPE> "Code type.").value_parser(value_parser!(u64)),
                )
                .arg(
                    arg!(-b --bin_path  <BIN_PATH> "Bin path.").value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-i --input_path  <INPUT_PATH> "Input path.")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-o --output_path  <OUTPUT_PATH> "Output path.")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-e --error_path  <ERROR_PATH> "Error output path.")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-r --real_time_limit  <REAL_TIME_LIMIT> "Real time limit.")
                        .value_parser(value_parser!(u64)),
                )
                .arg(
                    arg!(-c --cpu_time_limit <CPU_TIME_LIMIT> "CPU time limit.")
                        .value_parser(value_parser!(u64)),
                )
                .arg(
                    arg!(-m --max_memory <MAX_MEMORY> "Max memory.")
                        .value_parser(value_parser!(u64)),
                )
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
                .arg(arg!(--env <ENV> "Envs.").value_parser(value_parser!(String))),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("grpc", grpc_matches)) => {
            let addr = grpc_matches.get_one::<String>("ADDR").unwrap().as_str();
            println!("GreeterServer listening on {}", addr);

            Server::builder()
                .add_service(grpc::judger::judger_server::JudgerServer::new(
                    grpc::MyJudger::default(),
                ))
                .serve(addr.parse().unwrap())
                .await?;
        }
        Some(("cmd", cmd_matches)) => {
            let config = parse_cmd_config(&cmd_matches);
            println!("{:?}", config);

            let raw_judge_result = runner::run(&config).unwrap().unwrap();
            println!("{:?}", result::infer_result(&config, &raw_judge_result));
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    };
    Ok(())
}

fn parse_cmd_config(matches: &clap::ArgMatches) -> Config {
    let mut config = Config::default();

    if matches.contains_id("code_type") {
        let code_type = matches.get_one::<u64>("code_type").unwrap();
        config.code_type = match code_type {
            1 => Langs::C,
            2 => Langs::CPP,
            3 => Langs::GOLANG,
            _ => Langs::GENERAL,
        };
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
