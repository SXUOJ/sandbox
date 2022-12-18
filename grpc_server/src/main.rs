use judge::judger::judger_server::JudgerServer;
use tonic::transport::Server;

mod config;
mod judge;
mod langs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(JudgerServer::new(judge::MyJudger::default()))
        .serve(addr)
        .await?;

    Ok(())
}
