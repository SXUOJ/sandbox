use judger::{judger_server::Judger, JudgeReply, JudgeRequest, PingRequest, PongReply};
use tonic::{Request, Response, Status};

pub mod judger {
    tonic::include_proto!("judger");
}

#[derive(Default)]
pub struct MyJudger {}

#[tonic::async_trait]
impl Judger for MyJudger {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PongReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        Ok(Response::new(PongReply {
            message: "pong".to_string(),
        }))
    }

    async fn judge(&self, request: Request<JudgeRequest>) -> Result<Response<JudgeReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        use crate::config::Config;
        use judge_core::runner::run;

        let req = request.into_inner();
        let req_config = Config {
            code_type: Some(&req.r#type),
            source: req.source,
            input: req.input,
            output: req.output,
            time_limit: req.time_limit,
            memory_limit: req.memory_limit,
        };

        let (compile_config, run_config) = req_config.init().unwrap();

        //TODO: Add server for judge
        run(&compile_config).unwrap();

        run(&run_config).unwrap();

        Ok(Response::new(JudgeReply {
            message: format!("{:?}", compile_config),
        }))
    }
}
