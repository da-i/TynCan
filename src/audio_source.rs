use alsa::card::{Card, Iter as CardIter};

#[derive(Debug, Clone)]
pub struct AudioDeviceInfo {
    pub card: alsa::card::Card,
    pub name: String,
    pub longname: String,
    pub index: i32,
}

impl std::fmt::Display for AudioDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}: {} - {}", self.index, self.name, self.longname)
    }
}

/// Collect all available audio devices with their information
pub fn collect_audio_devices() -> Result<Vec<AudioDeviceInfo>, Box<dyn std::error::Error>> {
    let cards = list_audio_cards()?;
    let mut devices = Vec::new();
    
    for card in cards {
        let idx = card.get_index();
        let name = card.get_name().unwrap_or_else(|_| "Unknown".to_string());
        let longname = card.get_longname().unwrap_or_else(|_| "Unknown".to_string());
        
        devices.push(AudioDeviceInfo {
            card,
            name,
            longname,
            index: idx,
        });
    }
    
    Ok(devices)
}

/// List all available ALSA audio cards in the system
pub fn list_audio_cards() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let mut cards = Vec::new();
    
    for card_result in CardIter::new() {
        match card_result {
            Ok(card) => cards.push(card),
            Err(e) => {
                eprintln!("Warning: Failed to read audio card: {}", e);
                continue;
            }
        }
    }
    
    Ok(cards)
}

/// Get detailed information about a specific audio card
pub fn get_card_info(card: &Card) -> Result<CardInfo, Box<dyn std::error::Error>> {
    let index = card.get_index();
    let name = card.get_name().unwrap_or_else(|_| "Unknown".to_string());
    let longname = card.get_longname().unwrap_or_else(|_| "Unknown".to_string());
    
    // Driver information is not available in this ALSA version
    let driver = "Unknown".to_string();
    
    Ok(CardInfo {
        index,
        name,
        longname,
        driver,
    })
}

/// Information about an audio card
#[derive(Debug, Clone)]
pub struct CardInfo {
    pub index: i32,
    pub name: String,
    pub longname: String,
    pub driver: String,
}

impl std::fmt::Display for CardInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}: {} ({})", self.index, self.name, self.driver)
    }
}

/// Test if a card supports PCM playback
pub fn test_pcm_playback(card_index: i32) -> bool {
    let pcm_name = format!("hw:{}", card_index);
    alsa::pcm::PCM::new(&pcm_name, alsa::Direction::Playback, false).is_ok()
}

/// Test if a card supports PCM capture
pub fn test_pcm_capture(card_index: i32) -> bool {
    let pcm_name = format!("hw:{}", card_index);
    alsa::pcm::PCM::new(&pcm_name, alsa::Direction::Capture, false).is_ok()
}

/// Legacy function for backward compatibility
pub fn select_audio_card(cards: Vec<&Card>) -> Result<Card, Box<dyn std::error::Error>> {
    if cards.is_empty() {
        return Err("No audio cards available".into());
    }
    
    // Return the first available card (this is now mainly for compatibility)
    let card = cards.into_iter().next().cloned().ok_or("No audio cards available")?;
    Ok(card)
}