use clap::Parser;
use clap::{arg, command};
use sandbox::{grpc_judger, MyJudger};
use tonic::transport::Server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    addr: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    println!("GreeterServer listening on {}", args.addr);

    Server::builder()
        .add_service(grpc_judger::judger_server::JudgerServer::new(
            MyJudger::default(),
        ))
        .serve(args.addr.parse().unwrap())
        .await
        .unwrap();
}
