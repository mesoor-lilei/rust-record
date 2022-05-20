mod proto;

use std::env;

use proto::echo::{echo_service_client::EchoServiceClient, Text};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = EchoServiceClient::connect("http://[::1]:8080").await?;
    for content in env::args().skip(1) {
        let request = tonic::Request::new(Text { content });
        let response = client.echo_text(request).await?;
        println!("{}", response.get_ref().content);
    }
    Ok(())
}
