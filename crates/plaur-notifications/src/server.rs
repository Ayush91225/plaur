use plaur_common::{config::Config, error::Result};

pub async fn start(_config: Config) -> Result<()> {
    println!("Notification service started");
    Ok(())
}