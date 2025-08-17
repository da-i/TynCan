use alsa::card::{Card, Iter as CardIter};

pub fn list_audio_cards() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let mut cards = Vec::new();
    for card_result in CardIter::new() {
        let card = card_result?;
        cards.push(card);
    }
    Ok(cards)
}


pub fn select_audio_card(cards: Vec<&Card>) -> Result<Card, Box<dyn std::error::Error>> {
    // Implement the logic to select the audio card
    if cards.is_empty() {
        return Err("No audio cards available".into());
    }
    let card = cards.into_iter().next().cloned().ok_or("No audio cards available")?;

    Ok(card)
}