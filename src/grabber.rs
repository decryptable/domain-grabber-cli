use debugoff::multi_ptraceme_or_die;
use obfstr::obfstring;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::Client;
use std::error::Error;

pub async fn fetch_domains(date: &str) -> Result<String, Box<dyn Error>> {
    multi_ptraceme_or_die();
    let api_key = obfstring!("40fe311a2d67c99497d480a1434a9089");

    let url = format!(
        "{}{}{}{}/",
        obfstring!("https://domains-monitor.com/api/v1/"),
        api_key,
        obfstring!("/historical/dailyupdate/"),
        date
    );

    let client = Client::new();
    let response = client.get(&url).send().await?;

    let body = response.text().await?;

    if body.trim_start().starts_with("<?xml") {
        let mut reader = Reader::from_str(&body);

        let mut buf = Vec::new();
        let mut error_message = None;

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) if e.name().as_ref() == b"error" => {
                    if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                        error_message = Some(text.unescape().unwrap_or_default().into_owned());
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        if let Some(msg) = error_message {
            return Err(format!("{}", msg).into());
        }
    }

    Ok(body)
}
