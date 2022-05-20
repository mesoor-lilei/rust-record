mod proto;

use tonic::{async_trait, transport::Server, Request, Response, Status};

use proto::echo::{
    echo_service_server::{EchoService, EchoServiceServer},
    Text,
};

struct Service;

#[async_trait]
impl EchoService for Service {
    async fn echo_text(&self, request: Request<Text>) -> Result<Response<Text>, Status> {
        let content = &request.get_ref().content;
        println!("{}", content);
        Ok(Response::new(Text {
            content: format!("[{}]", content),
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Server::builder()
        .add_service(EchoServiceServer::new(Service))
        .serve("[::1]:8080".parse()?)
        .await?;
    Ok(())
}
