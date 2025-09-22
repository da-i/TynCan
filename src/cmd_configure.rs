use crate::audio_source::{collect_audio_devices, AudioDeviceInfo};
use crate::constants::*;
use dialoguer::{Select, theme::ColorfulTheme};
use console::{style, Term};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;
use std::process::Command;



fn download_darkice() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Downloading DarkIce...").bold().green());
    let mut response = reqwest::blocking::get(DARKICE_SOURCE)?;
    let mut file = std::fs::File::create("darkice.deb")?;
    std::io::copy(&mut response, &mut file)?;

    println!("{}", style("DarkIce downloaded as darkice.deb").bold().green());
    Ok(())
}

fn install_packages_via_apt(packages: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if packages.is_empty() {
        return Ok(());
    }
    
    let package_list = packages.join(" ");
    println!("{}", style(&format!("Installing packages: {} via apt-get...", package_list)).bold().green());
    
    // Check if running as root or with sudo
    let mut command = if std::env::var("USER").unwrap_or_default() == "root" {
        Command::new("apt-get")
    } else {
        let mut cmd = Command::new("sudo");
        cmd.arg("apt-get");
        cmd
    };
    
    let mut cmd = command
        .arg("install")
        .arg("-y"); // Auto-confirm installation
    
    // Add all packages to the command
    for package in packages {
        cmd = cmd.arg(package);
    }
    
    let output = cmd.output()?;
    
    if output.status.success() {
        println!("{}", style(&format!("✅ {} installed successfully!", package_list)).bold().green());
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        println!("{}", style(&format!("❌ Failed to install {}", package_list)).bold().red());
        println!("Error: {}", error_msg);
        Err(format!("Failed to install packages {}: {}", package_list, error_msg).into())
    }
}

fn verify_icecast2() -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder: Implement actual verification logic if needed
    println!("{}", style("Verifying Icecast2 installation...").bold().cyan());
    // For now, just return true

    Ok(())
}


fn verify_file_hash(file_path: &str, expected_hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
    println!("{}", style(&format!("Verifying SHA-256 hash for {}...", file_path)).bold().cyan());
    
    // Read the file contents
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    
    // Calculate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let result = hasher.finalize();
    let calculated_hash = format!("{:x}", result);
    
    let hashes_match = calculated_hash.eq_ignore_ascii_case(expected_hash);
    
    if hashes_match {
        println!("{}", style("✅ Hash verification successful!").bold().green());
        println!("Expected: {}", style(expected_hash).dim());
        println!("Actual:   {}", style(&calculated_hash).dim());
    } else {
        println!("{}", style("❌ Hash verification failed!").bold().red());
        println!("Expected: {}", style(expected_hash).dim());
        println!("Actual:   {}", style(&calculated_hash).dim());
    }
    
    Ok(hashes_match)
}

fn verify_darkice_file() -> Result<bool, Box<dyn std::error::Error>> {
    if !std::path::Path::new("darkice.deb").exists() {
        return Err("darkice.deb file not found".into());
    }
    
    let verification_result = verify_file_hash("darkice.deb", DARKICE_HASH)?;
    
    if verification_result {
        println!("{}", style("✅ DarkIce file verification successful!").bold().green());
    } else {
        println!("{}", style("❌ DarkIce file verification failed!").bold().red());
        return Err("Downloaded file hash does not match expected value".into());
    }
    
    Ok(verification_result)
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

fn select_audio_device(auto: bool) -> Result<Option<AudioDeviceInfo>, Box<dyn std::error::Error>> {
    println!("Scanning for available audio devices...");
    let devices = collect_audio_devices()?;

    if devices.is_empty() {
        println!("{}", style("❌ No audio devices found!").red());
        return Ok(None);
    }

    println!("Found {} audio device(s):\n", devices.len());

    let selected_device = if auto {
        println!("{}", style("Auto mode: selecting first available device").yellow());
        devices[0].clone()
    } else {
        let device_strings: Vec<String> = devices.iter()
            .map(|device| device.to_string())
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an audio device")
            .default(0)
            .items(&device_strings)
            .interact()?;

        devices[selection].clone()
    };

    display_device_details(&selected_device)?;
    Ok(Some(selected_device))
}

fn confirm_device_selection(device: &AudioDeviceInfo, auto: bool) -> Result<bool, Box<dyn std::error::Error>> {
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
        println!("Selected device: {}", style(&device.to_string()).cyan());
        
        // TODO: Save configuration to file
        println!("\n{}", style("Next steps:").bold());
        println!("- Use '{} start --device {}' to start with this device", APP_NAME.to_lowercase(), device.index);
        println!("- Configuration will be saved for future use");
    } else {
        println!("{}", style("❌ Configuration cancelled.").yellow());
    }
    
    Ok(continue_selection)
}

pub fn run_configure(auto: bool) -> Result<(), Box<dyn std::error::Error>> {
    
    let _term = initialize_configuration()?;
    
    println!("Downloading required files and packages...");
    let apt_packages = ["icecast2","libmp3lame0", "libtwolame0"];
    install_packages_via_apt(&apt_packages)?;
    download_darkice()?;
    verify_darkice_file()?;

    verify_icecast2()?;

    println!("All files downloaded successfully.\n");

    // Collect available audio devices, select one
    let selected_device = match select_audio_device(auto)? {
        Some(device) => device,
        None => return Ok(()),
    };

    // Ask if user wants to continue with this device and handle the response
    confirm_device_selection(&selected_device, auto)?;
    
    Ok(())
}
