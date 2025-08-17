use alsa::card::Card;

pub fn convert_card_to_stream(card: Card, format: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    


    let format = format.unwrap_or_else(|| "mp3".to_string());

    println!("Creating stream from: {:?}", card.get_longname());
    println!("Format selected: {}", format);
    let stream = "blabla".to_string();

    Ok(stream)
}

