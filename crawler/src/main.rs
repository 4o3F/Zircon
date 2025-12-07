use tracing::{Level, error};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Set default subscriber failed");

    if let Err(e) = dotenv::dotenv() {
        error!("Environment variable load failed! {}", e);
        return;
    }

    
}
