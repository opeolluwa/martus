use martus_auth::auth_server::AuthServer;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tonic::transport::Server;
mod grpc_server;
pub mod martus_auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SocketAddr::from(([0, 0, 0, 0], 5001));
    let server = grpc_server::GrpcServer::default();

    let database_connection_url = std::env::var("DATABASE_URL")?;
    let database_pool_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_connection_url);

    //run  migrations
    sqlx::migrate!("./migrations")
        .run(&database_pool_connection)
        .await?;
    
    // run the gRPC server
    Server::builder()
        .add_service(AuthServer::new(server))
        .serve(server_address)
        .await?;

    Ok(())
}
