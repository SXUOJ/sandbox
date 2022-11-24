use grpc::judger::greeter_server::GreeterServer;
use grpc::judger::judger_server::JudgerServer;
use tonic::transport::Server;

mod grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(grpc::greeter::MyGreeter::default()))
        .add_service(JudgerServer::new(grpc::judge::MyJudger::default()))
        .serve(addr)
        .await?;

    Ok(())
}
