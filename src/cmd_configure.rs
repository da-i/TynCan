use crate::audio_source::{collect_audio_devices, AudioDeviceInfo};
use crate::constants::*;
use dialoguer::{Select, theme::ColorfulTheme};
use console::{style, Term};


fn check_darkice_link() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Checking DarkIce download link...").bold().green());
    let response = reqwest::blocking::get(DARKICE_SOURCE)?;
    if response.status().is_success() {
        println!("{}", style("DarkIce download link is valid.").bold().green());
        Ok(())
    } else {
        println!("{}", style("Warning: DarkIce download link is not reachable!").bold().red());
        Err("DarkIce download link is not reachable".into())
    }
}

fn download_darkice() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Downloading DarkIce...").bold().green());
    let mut response = reqwest::blocking::get(DARKICE_SOURCE)?;
    let mut file = std::fs::File::create("darkice.deb")?;
    std::io::copy(&mut response, &mut file)?;

    println!("{}", style("DarkIce downloaded as darkice.deb").bold().green());
    Ok(())
}

fn initialize_configuration() -> Result<Term, std::io::Error> {
    // Placeholder for future configuration initialization logic
    let term = Term::stdout();
    term.clear_screen()?;
    
    println!("{}", style(&format!("🎵 {} Audio Device Configuration", APP_NAME)).bold().green());
    println!("{}", style("=====================================").green());
    println!();
    println!("{}", style("This utility will help you configure a raspberry pi for TynCan.").bold());

    Ok(term)
}


fn check_download_links() -> Result<(), Box<dyn std::error::Error>> {
    check_darkice_link()?;
    Ok(())
}

fn download_links() -> Result<(), Box<dyn std::error::Error>> {
    download_darkice()?;
    Ok(())
}

fn display_device_details(device: &AudioDeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("=== Selected Audio Device Details ===").bold().cyan());
    println!("{}: {}", style("Card Index").bold(), device.index);
    println!("{}: {}", style("Short Name").bold(), device.name);
    println!("{}: {}", style("Long Name").bold(), device.longname);
    
    // Try to get more detailed information about the card
    if let Ok(_mixers) = alsa::mixer::Mixer::new(&format!("hw:{}", device.index), false) {
        println!("{}: Available", style("Mixer Support").bold());
        
        // List available PCM devices
        if let Ok(_ctl) = alsa::ctl::Ctl::new(&format!("hw:{}", device.index), false) {
            println!("{}: Available", style("Control Interface").bold());
            
            // Try to get PCM info
            let mut pcm_devices = Vec::new();
            for device_num in 0..8 { // Check first 8 devices
                let pcm_name = format!("hw:{},{}", device.index, device_num);
                if let Ok(_pcm) = alsa::pcm::PCM::new(&pcm_name, alsa::Direction::Playback, false) {
                    pcm_devices.push(format!("Device {}: {}", device_num, pcm_name));
                }
            }
            
            if !pcm_devices.is_empty() {
                println!("{}: ", style("PCM Playback Devices").bold());
                for pcm_device in pcm_devices {
                    println!("  - {}", pcm_device);
                }
            }
        }
    } else {
        println!("{}: Not available", style("Mixer Support").bold());
    }
    
    println!();
    Ok(())
}

pub fn run_configure(auto: bool) -> Result<(), Box<dyn std::error::Error>> {
    
    let term = initialize_configuration()?;
    
    check_download_links()?;

    println!("All links are reachable.\n");

    println!("Downloading DarkIce...");
    download_links()?;

    // Collect available audio devices
    println!("Scanning for available audio devices...");
    let devices = collect_audio_devices()?;
    
    if devices.is_empty() {
        println!("{}", style("❌ No audio devices found!").red());
        return Ok(());
    }
    
    println!("Found {} audio device(s):\n", devices.len());
    
    let selected_device = if auto {
        // Auto mode: select the first device
        println!("{}", style("Auto mode: selecting first available device").yellow());
        &devices[0]
    } else {
        // Interactive mode: let user choose
        let device_strings: Vec<String> = devices.iter()
            .map(|device| device.to_string())
            .collect();
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an audio device")
            .default(0)
            .items(&device_strings)
            .interact()?;
        
        &devices[selection]
    };
    
    // Display detailed information about the selected device
    display_device_details(selected_device)?;
    
    // Ask if user wants to continue with this device (skip in auto mode)
    let continue_selection = if auto {
        true
    } else {
        dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue with this audio device?")
            .default(true)
            .interact()?
    };
    
    if continue_selection {
        println!("{}", style("✅ Audio device configuration complete!").green());
        println!("Selected device: {}", style(&selected_device.to_string()).cyan());
        
        // TODO: Save configuration to file
        println!("\n{}", style("Next steps:").bold());
        println!("- Use '{} start --device {}' to start with this device", APP_NAME.to_lowercase(), selected_device.index);
        println!("- Configuration will be saved for future use");
    } else {
        println!("{}", style("❌ Configuration cancelled.").yellow());
    }
    
    Ok(())
}
