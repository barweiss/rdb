extern crate rdb;
mod op;
mod plan;
mod row;
mod value;

use log::trace;
use op::Op;
use plan::plan_statement;
use sqlparser::ast::TableFactor;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};

use rdb::relational::rdb_server::{Rdb, RdbServer};
use rdb::relational::{RunRequest, RunResponse};

#[derive(Debug)]
pub struct Impl {
}

impl Impl {
}

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

        for statement in ast {
            let op = plan_statement(&statement);
            op.execute();
        }

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
