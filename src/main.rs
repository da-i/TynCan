

fn main() {
    // Make sure to add 'regex' to Cargo.toml dependencies
    let devices_result = list_audio_cards();
    match devices_result {
        Ok(devices) => {
            if devices.is_empty() {
                println!("No USB audio devices found.");
            } else {
                println!("USB audio devices:");
                for device in devices {
                    println!(
                        "Card {} ({}): Device {} ({}), Description: {}, Subdevices: {}",
                        device.card, device.card_name, device.device, device.device_name, device.description, device.subdevices.join(", ")
                    );
                }
            }
        }
        Err(e) => {
            println!("Error listing audio devices: {}", e);
        }
    }
}
mod audio_source;
use audio_source::{list_audio_cards};
