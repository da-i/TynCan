use regex::Regex;

#[derive(Debug)]
pub struct AudioDevice {
    pub card: u32,
    pub device: u32,
    pub card_name: String,
    pub device_name: String,
    pub description: String,
    pub subdevices: Vec<String>,
}

pub fn process_arecord_stdout(arecord_stdout: &[u8]) -> Result<Vec<AudioDevice>, Box<dyn std::error::Error>> {
    let stdout = String::from_utf8_lossy(arecord_stdout);
    let mut devices = Vec::new();
    let mut first_loop = true;
    let mut subdevices: Vec<String> = Vec::new();
    let mut card: String = String::new();
    let mut card_name: String = String::new();
    let mut description: String = String::new();
    let mut device: String = String::new();
    let mut device_name: String = String::new();
    let mut device_desc: String = String::new();

    let card_device_re = Regex::new(
        r"card (\d+): ([^\[]+) \[([^\]]+)\], device (\d+): ([^\[]+) \[([^\]]+)\]"
    ).unwrap();

    let mut lines = stdout.lines();
    let first_line = lines.next().unwrap_or("");
    if first_line != "**** List of CAPTURE Hardware Devices ****" {
        println!("Unexpected output format from arecord: {}", first_line);
        panic!("Expected header line not found");
    }

    for line in lines {
        if line.starts_with("card") {
            if first_loop { 
                first_loop = false;
                continue
            }
            else {
                devices.push(AudioDevice {
                    card: card.parse().unwrap_or(0),
                    device: device.parse().unwrap_or(0),
                    card_name: card_name.trim().to_string(),
                    device_name: device_name.trim().to_string(),
                    description: format!("{} / {}", description, device_desc),
                    subdevices,
                });
            }
            subdevices = Vec::new();
            if let Some(caps) = card_device_re.captures(line) {
                card = caps.get(1).unwrap().as_str().to_string();
                card_name = caps.get(2).unwrap().as_str().trim().to_string();
                description = caps.get(3).unwrap().as_str().trim().to_string();
                device = caps.get(4).unwrap().as_str().to_string();
                device_name = caps.get(5).unwrap().as_str().trim().to_string();
                device_desc = caps.get(6).unwrap().as_str().trim().to_string();
            }
        } else {
            subdevices.push(line.trim().to_string());
        }
    }
    if !first_loop {
        devices.push(AudioDevice {
            card: card.parse().unwrap_or(0),
            device: device.parse().unwrap_or(0),
            card_name: card_name.trim().to_string(),
            device_name: device_name.trim().to_string(),
            description: format!("{} / {}", description, device_desc),
            subdevices,
        });
    }
    Ok(devices)
}

pub fn list_audio_cards() -> Result<Vec<AudioDevice>, Box<dyn std::error::Error>>{
    use std::process::Command;
    let output = Command::new("arecord")
        .arg("-l")
        .output()
        .expect("Failed to execute arecord");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let devices = process_arecord_stdout(stdout.as_bytes())?;
    Ok(devices)
}
