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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cards = audio_source::list_audio_cards()?;
    report_soundcards(&cards);
    let card = audio_source::select_audio_card(cards.iter().collect::<Vec<&alsa::Card>>())?;
    println!("Audio source selected: {:?}", card.get_longname());
    let stream = audio_stream::convert_card_to_stream(card, Some("mp3".to_string()))?;
    println!("Audio stream created successfully: {:?}", stream);
    Ok(())
}

mod audio_source;
mod audio_stream;
