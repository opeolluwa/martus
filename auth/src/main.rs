use martus_auth::auth_server::AuthServer;
use std::net::SocketAddr;
use tonic::transport::Server;

mod grpc_server;
pub mod martus_auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SocketAddr::from(([0, 0, 0, 0], 5001));
    let server = grpc_server::GrpcServer::default();

    Server::builder()
        .add_service(AuthServer::new(server))
        .serve(server_address)
        .await?;

    Ok(())
}
