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
        use judge_core::{
            result::{infer_result, Result},
            runner::run,
        };

        let req = request.into_inner();
        for i in 0..req.input.len() {
            let req_config = Config {
                code_type: Some(&req.r#type),
                source: req.source.clone(),
                input: req.input[i].clone(),
                output: req.output[i].clone(),
                time_limit: req.time_limit,
                memory_limit: req.memory_limit,
            };
            let (compile_config, run_config) = req_config.init().unwrap();

            let read_error =
                |error_file_path| -> String { std::fs::read_to_string(error_file_path).unwrap() };

            let mut compile_result =
                infer_result(&compile_config, &run(&compile_config).unwrap().unwrap());

            if compile_result.status != Result::Success {
                compile_result.status = Result::CompileError;
                compile_result.error = read_error(compile_config.error_path);
                return Ok(Response::new(JudgeReply {
                    message: format!("{}", compile_result),
                }));
            };

            let run_result = infer_result(&run_config, &run(&run_config).unwrap().unwrap());

            return Ok(Response::new(JudgeReply {
                message: format!("{:?}", run_result),
            }));
        }
        Ok(Response::new(JudgeReply {
            message: format!("OK"),
        }))
    }
}
