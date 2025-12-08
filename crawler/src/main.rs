#![deny(clippy::unwrap_used)]

mod mode;
mod utils;

use std::env;

use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, error};
use tracing_subscriber::FmtSubscriber;
use tracing_unwrap::ResultExt;

#[derive(clap::Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    mode: CrawlerMode,
}

#[derive(clap::Subcommand)]
enum CrawlerMode {
    Anime,
    User,
    Interaction,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Set default subscriber failed");

    let cli = Cli::parse();

    if let Err(e) = dotenv::dotenv() {
        error!("Environment variable load failed! {}", e);
        return;
    }

    let db_connect_url =
        env::var("DATABASE_URL").expect_or_log("Failed to get environment variable DATABASE_URL");
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_connect_url)
        .await
        .expect_or_log("Could not connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect_or_log("Failed to run database migration");

    match cli.mode {
        CrawlerMode::Anime => mode::anime::crawl_anime(&db).await,
        CrawlerMode::User => todo!(),
        CrawlerMode::Interaction => todo!(),
    }
}
