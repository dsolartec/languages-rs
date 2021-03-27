use languages_rs::*;

fn main() -> anyhow::Result<()> {
    // Create a new configuration
    let config = Config::new("examples/languages_toml", vec!["en", "es"])?;

    // Load the texts.
    let mut texts = load(config)?;

    // Get English texts.
    let texts_en = texts.try_get_language("en")?;

    if let Some(greeting_en) = texts_en.try_get_text("greeting") {
        println!("Greeting (English): {}", greeting_en);
    }

    if let Some(messages_en) = texts_en.try_get_text("messages") {
        let messages_en = messages_en.get_array().unwrap();

        println!("Messages length (English): {}", messages_en.len());
        println!("Message 1 (English): {}", &messages_en[0]);
        println!("Message 2 (English): {}", &messages_en[1]);
    }

    if let Some(pages_en) = texts_en.try_get_text("pages") {
        let pages_en = pages_en.get_object().unwrap();

        let home_page = pages_en.get("home").unwrap().get_object().unwrap();

        println!(
            "Home page title (English): {}",
            home_page.get("title").unwrap()
        );
        println!(
            "Home page description (English): {}",
            home_page.get("description").unwrap()
        );
    }

    println!();

    // Get Spanish texts.
    if let Some(greeting_es) = texts.try_get_text_from_language("es", "greeting")? {
        println!("Greeting (Spanish): {}", greeting_es);
    }

    if let Some(messages_es) = texts.try_get_text_from_language("es", "messages")? {
        let messages_es = messages_es.get_array().unwrap();

        println!("Messages length (Spanish): {}", messages_es.len());
        println!("Message 1 (Spanish): {}", &messages_es[0]);
        println!("Message 2 (Spanish): {}", &messages_es[1]);
    }

    if let Some(pages_es) = texts.try_get_text_from_language("es", "pages")? {
        let pages_es = pages_es.get_object().unwrap();

        let home_page = pages_es.get("home").unwrap().get_object().unwrap();

        println!(
            "Home page title (Spanish): {}",
            home_page.get("title").unwrap()
        );
        println!(
            "Home page description (Spanish): {}",
            home_page.get("description").unwrap()
        );
    }

    Ok(())
}
