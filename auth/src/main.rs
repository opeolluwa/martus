use martus_auth::auth_server::AuthServer;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tonic::transport::Server;

pub mod database;
mod grpc_server;
pub mod martus_auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //parse env
    dotenv::dotenv().ok();

    //enable logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let server_address = SocketAddr::from(([0, 0, 0, 0], 5001));
    let server = grpc_server::GrpcServer::default();

    let database_connection_url = std::env::var("DATABASE_URL")?;
    let _database_pool_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_connection_url);

    //TODO: run  migrations
    // sqlx::migrate!("./migrations")
    //     .run(&database_pool_connection)
    //     .await?;

    tracing::info!(message = "Starting server.", %server_address);

    // run the gRPC server
    Server::builder()
        .trace_fn(|_| tracing::info_span!("martus_auth_server"))
        .add_service(AuthServer::new(server))
        .serve(server_address)
        .await?;

    Ok(())
}
