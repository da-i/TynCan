use crate::constants::*;
use console::style;

pub fn run_status() -> Result<(), Box<dyn std::error::Error>> {
    print_app_info_default();

    println!("{}", style(&format!("🎵 {} Service Status", APP_NAME)).bold().green());
    println!("{}", style("========================").green());
    println!();
    
    // Check if any audio devices are available
    println!("🔍 Checking system status...");
    underdevelopment_notice();
    // let devices = collect_audio_devices()?;
    
    print_app_info_detailed();
    Ok(())
}
