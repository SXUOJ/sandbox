use super::judger;
use judger::{judger_server::Judger, JudgeReply, JudgeRequest};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MyJudger {}

#[tonic::async_trait]
impl Judger for MyJudger {
    async fn judge(&self, request: Request<JudgeRequest>) -> Result<Response<JudgeReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = JudgeReply {
            message: format!("Hello {:?}!", request.into_inner()),
        };
        //TODO: Add server for judge
        Ok(Response::new(reply))
    }
}
