extern crate rdb;

mod plan;
mod storage;

use crate::plan::Planner;
use sqlparser::dialect::{self};
use sqlparser::parser::Parser;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};

use rdb::relational::rdb_server::{Rdb, RdbServer};
use rdb::relational::{RunRequest, RunResponse};

#[derive(Debug, Default)]
pub struct Impl {
    planner: dyn Planner,
}

impl Default for Impl {
    fn default() -> Self {
        return Impl {};
    }
}

#[tonic::async_trait]
impl Rdb for Impl {
    type RunStream = ReceiverStream<Result<RunResponse, Status>>;

    async fn run(&self, request: Request<RunRequest>) -> Result<Response<Self::RunStream>, Status> {
        let ast = Parser::parse_sql(&dialect::GenericDialect {}, &request.get_ref().sql)
            .map_err(|err| Status::invalid_argument(format!("parsing sql: {err}")))?;
        let plan = self.planner.plan(ast);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = Impl::default();

    Server::builder()
        .add_service(RdbServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
