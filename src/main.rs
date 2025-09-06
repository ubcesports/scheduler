use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use scheduler::{create_router, ApplicationData, Config};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let config = Config::read()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let addr = SocketAddr::from((
        IpAddr::from_str(&config.http.listen.address)?,
        config.http.listen.port,
    ));

    let router = create_router(ApplicationData { config, pool });
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on {addr}");

    axum::serve(listener, router).await?;
    Ok(())
}
