use std::net::SocketAddr;

use anyhow::Result;
use services::create_app;

#[tokio::main]
async fn main() -> Result<()> {
    
    dotenvy::dotenv().expect(".env file not found");

    //initialize the database
    database::init_db().await?;

    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_line_number(true)
        ).init();

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    axum::Server::bind(&addr)
        .serve(create_app().into_make_service())
        .await?;
    Ok(())
}
