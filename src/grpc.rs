use super::Config;
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
        println!("Got a judge request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let mut results: Vec<judger::Result> = vec![];

        let mut judger = crate::judger::Judger {
            submit_id: req.submit_id.clone(),
            code_type: req.r#type,
            source: req.source.clone(),
            time_limit: req.time_limit,
            memory_limit: req.memory_limit,
            samples: vec![],
            base_dir: String::new().into(),
            compile_config: Config::default(),
        };

        for sample in req.samples {
            judger.samples.push(sample.into());
        }

        let run_results = judger.run();

        for result in run_results {
            results.push(judger::Result {
                status: result.status.into(),
                signal: result.signal,
                real_time: result.real_time,
                cpu_time: result.cpu_time,
                memory: result.memory,
                error: result.error,
            });
        }

        println!("{:?}", results);

        Ok(Response::new(JudgeReply {
            submit_id: req.submit_id,
            results,
        }))
    }
}

impl From<judger::Sample> for crate::judger::Sample {
    fn from(value: judger::Sample) -> Self {
        Self {
            input: value.input,
            output: value.output,
        }
    }
}
