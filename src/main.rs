use std::{fs, env};
use shellexpand;
use chrono::Datelike;
use textwrap;
use term_size;

fn get_quotes(path: &str) -> Vec<String> {

    fs::read_to_string(shellexpand::tilde(path).to_string())
        .unwrap_or("No quotes file found".to_string()).lines()
        .map(|str| str.to_string())
        .collect()

}

fn get_todays_quote() -> String {
    let today = chrono::offset::Local::now().day();
    let quotes_path = match env::var("LOVESAY_PATH") {
        Ok(str) => str.to_string(),
        Err(_) => "~/.config/lovesay/quotes".to_string(),
    };
    let quotes = get_quotes(&quotes_path);

    match quotes.get(today as usize) {
        Some(str) => String::from(str),
        None => String::from("No quotes for today"),
    }
}

fn get_quote_vec() -> Vec<String> {
    let today_quote = get_todays_quote();
    let width = match term_size::dimensions() {
        Some((width, _)) => width,
        None => 80,
    };

    textwrap::wrap(&today_quote, width - 25)
        .iter()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let quote_vec = get_quote_vec();

    println!("{:?}", quote_vec);
}
