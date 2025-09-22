use crate::constants::*;
use console::style;

pub fn run_start(device: Option<i32>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style(&format!("🎵 Starting {} Audio Service", APP_NAME)).bold().green());
    println!("{}", style("===============================").green());
    println!("Selected device: {:?}", device);
    println!("Port: {}", port);
    underdevelopment_notice();

    Ok(())
}
