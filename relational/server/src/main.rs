extern crate rdb;

use std::env;

use log::trace;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};

use rdb::relational::rdb_server::{Rdb, RdbServer};
use rdb::relational::{RunRequest, RunResponse};

#[derive(Debug, Default)]
pub struct Impl {}

#[tonic::async_trait]
impl Rdb for Impl {
    type RunStream = ReceiverStream<Result<RunResponse, Status>>;

    async fn run(&self, request: Request<RunRequest>) -> Result<Response<Self::RunStream>, Status> {
        let dialect = GenericDialect {};
        let request = request.get_ref();
        let sql = &request.sql;
        let params = &request.parameters;
        trace!("sql={sql}, params={params:?}");
        let ast = Parser::parse_sql(&dialect, sql)
            .map_err(|err| Status::invalid_argument(err.to_string()))?;
        trace!("{ast:?}");

        Err(Status::unimplemented("not yet implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr = "[::1]:50051".parse()?;
    let server = Impl::default();

    Server::builder()
        .add_service(RdbServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
