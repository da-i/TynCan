

fn main() {
    match audio_source::list_audio_cards() {
        Ok(cards) => {
            if cards.is_empty() {
                println!("No ALSA cards found.");
            } else {
                println!("ALSA cards:");
                for card in cards {
                    let idx = card.get_index();
                    let name = card.get_name().unwrap_or_else(|_| "Unknown".to_string());
                    let longname = card.get_longname().unwrap_or_else(|_| "Unknown".to_string());
                    println!("~ Card {}: {} - {}", idx, name, longname);
                }
            }
        }
        Err(e) => {
            println!("Error listing ALSA cards: {}", e);
        }
    }
}
mod audio_source;
