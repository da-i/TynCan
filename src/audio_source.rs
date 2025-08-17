use alsa::card::{Card, Iter as CardIter};

pub fn list_audio_cards() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let mut cards = Vec::new();
    for card_result in CardIter::new() {
        let card = card_result?;
        cards.push(card);
    }
    Ok(cards)
}
