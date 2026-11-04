//! SigmaOS-anomaly-detection - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-anomaly-detection");
    println!("SigmaOS SigmaOS-anomaly-detection v0.1.0");
    Ok(())
}
