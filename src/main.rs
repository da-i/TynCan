fn report_soundcards(cards: &[alsa::card::Card]) {
    if cards.is_empty() {
        println!("No ALSA cards found.");
    } else {
        println!("ALSA cards:");
        for card in cards {
            let idx = card.get_index();
            let name = card.get_name().unwrap_or_else(|_| "Unknown".to_string());
            let longname = card.get_longname().unwrap_or_else(|_| "Unknown".to_string());
            println!("Card {}: {} - {}", idx, name, longname);
        }
    }
}

fn main() {
    let audio_cards = audio_source::list_audio_cards();
    let cards = match audio_cards {
        Ok(c) => c,
        Err(e) => {
            println!("Error listing ALSA sound cards: {}", e);
            Vec::new()
        }
    };
    report_soundcards(&cards);
    match audio_source::select_audio_card(cards.iter().cloned().collect()) {
        Ok(source) => {
            // Use the selected audio source here
            println!("Audio source selected: {:?}", source.get_longname());
        }
        Err(e) => {
            println!("Error selecting audio source: {}", e);
        }
    }
}

mod audio_source;
