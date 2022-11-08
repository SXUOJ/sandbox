use clap::{Arg, ArgAction, Command};

mod killer;

fn main() {
    let cmd = Command::new("Judger")
        .author("ther")
        .about("A judger for SXU Online Judge")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("test")
                .short_flag('T')
                .long_flag("test")
                .about("Just a test...")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("test", test_matchs)) => {
            if test_matchs.contains_id("name") {
                let name = test_matchs.get_one::<String>("name").unwrap();
                println!("name = {}", name);
            }
        }
        _ => println!("Oh..."),
    }
}
