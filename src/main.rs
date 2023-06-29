use atty::Stream;
use chrono::Datelike;
use kolorz::Kolor;
use shellexpand;
use std::{env, fs, io};
use term_size;
use textwrap;

fn main() {
    let heart = match env::var("LOVESAY_NO_NERD") {
        Ok(_) => "\u{2665}",
        Err(_) => "\u{f004}",
    };

    let kolor = match env::var("LOVESAY_COLORSCHEME") {
        Ok(str) => Kolor::new(&str),
        Err(_) => Kolor::new(""),
    };

    let today_quote = get_todays_quote();
    let quote_vec = get_quote_vec(today_quote);

    let hearts = (
        kolor.red(heart),
        kolor.purple(heart),
        kolor.blue(heart),
        kolor.green(heart),
        kolor.orange(heart),
        kolor.yellow(heart),
    );

    let mut printable_quotes: Vec<String> = vec![String::new(); 5];
    for (i, quote) in quote_vec.iter().enumerate() {
        if i < 5 {
            printable_quotes[i] = format!("{} {} {}", hearts.0, kolor.white(quote), hearts.0);
        } else {
            printable_quotes.push(format!("{} {} {}", hearts.0, kolor.white(quote), hearts.0));
        }
    }

    println!("   {} {}   {} {}   ", hearts.0, hearts.0, hearts.0, hearts.0);
    println!(" {}     {}     {}      {}", hearts.1, hearts.1, hearts.1, printable_quotes[0]);
    println!(" {}           {}      {}", hearts.2, hearts.2, printable_quotes[1]);
    println!("   {}       {}        {}", hearts.3, hearts.3, printable_quotes[2]);
    println!("     {}   {}          {}", hearts.4, hearts.4, printable_quotes[3]);
    println!("       {}            {}", hearts.5, printable_quotes[4]);

    for quote in quote_vec.iter().skip(5) {
        println!("                    {} {} {}", hearts.0, kolor.white(quote), hearts.0);
    }
}

fn get_todays_quote() -> String {
    if atty::is(Stream::Stdin) {
        let argv: Vec<String> = env::args().skip(1).collect();
        match argv.len() > 0 {
            true => argv.join(" "),
            false => get_todays_quote_from_file(),
        }
    } else {
        io::read_to_string(io::stdin())
            .unwrap_or_else(|_| "Couldn't read from stdin".to_string())
            .trim_end()
            .to_string()
    }
}

fn get_todays_quote_from_file() -> String {
    let today = chrono::offset::Local::now().day();
    let quotes_path = match env::var("LOVESAY_PATH") {
        Ok(str) => str,
        Err(_) => "~/.config/lovesay/quotes".to_string(),
    };

    let quotes: Vec<String> = match fs::read_to_string(shellexpand::tilde(&quotes_path).as_ref()) {
        Ok(ret) => ret.lines().map(String::from).collect(),
        Err(_) => return "No quotes file found".to_string(),
    };

    match quotes.get(today as usize - 1) {
        Some(str) => str.to_string(),
        None => "No quotes for today".to_string(),
    }
}

fn get_quote_vec(today_quote: String) -> Vec<String> {
    let max_width = env::var("LOVESAY_MAX_WIDTH")
        .ok()
        .and_then(|s| s.parse::<usize>().ok());
    let term_width = term_size::dimensions().map(|(w, _)| w);
    let default_width = Some(80);

    let width = max_width.or(term_width).or(default_width).unwrap();

    if width < 25 {
        return vec![];
    }

    textwrap::wrap(&today_quote, width - 25)
        .iter()
        .map(|line| line.to_string())
        .collect()
}
