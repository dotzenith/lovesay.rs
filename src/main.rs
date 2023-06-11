use atty::Stream;
use chrono::Datelike;
use shellexpand;
use std::{env, fs, io};
use term_size;
use textwrap;

fn main() {
    let today_quote = get_todays_quote();
    let quote_vec = get_quote_vec(today_quote);

    // Hearts
    let oneheart: String = "[38;2;243;139;168m\u{f004}[0m".to_string();
    let twoheart: String = "[38;2;203;166;247m\u{f004}[0m".to_string();
    let threeheart: String = "[38;2;137;180;250m\u{f004}[0m".to_string();
    let fourheart: String = "[38;2;166;227;161m\u{f004}[0m".to_string();
    let fiveheart: String = "[38;2;250;179;135m\u{f004}[0m".to_string();
    let sixheart: String = "[38;2;249;226;175m\u{f004}[0m".to_string();

    // Quote lines
    let empty_string = "".to_string();
    let mut printable_quotes: Vec<String> = vec![empty_string; 5];
    for (i, quote) in quote_vec.iter().enumerate() {
        if i < 5 {
            printable_quotes[i] = format!("{oneheart} {quote} {oneheart}");
        } else {
            printable_quotes.push(format!("{oneheart} {quote} {oneheart}"));
        }
    }

    // Heart
    println!("   {oneheart} {oneheart}   {oneheart} {oneheart}   ");
    println!(" {twoheart}     {twoheart}     {twoheart}      {}", printable_quotes[0]);
    println!(" {threeheart}           {threeheart}      {}", printable_quotes[1]);
    println!("   {fourheart}       {fourheart}        {}", printable_quotes[2]);
    println!("     {fiveheart}   {fiveheart}          {}", printable_quotes[3]);
    println!("       {sixheart}            {}", printable_quotes[4]);
}

fn get_quotes(path: &str) -> Vec<String> {
    fs::read_to_string(shellexpand::tilde(path).to_string())
        .unwrap_or("No quotes file found".to_string())
        .lines()
        .map(|str| str.to_string())
        .collect()
}

fn get_todays_quote() -> String {
    if atty::is(Stream::Stdin) {
        let argv: Vec<_> = env::args().map(|v| v.to_owned()).collect();
        match argv.len() > 1 {
            true => argv[1..].join(" "),
            false => get_todays_quote_from_file(),
        }
    } else {
        io::read_to_string(io::stdin())
            .unwrap_or("Couldn't read from stdin".to_string())
            .trim_end()
            .to_string()
    }
}

fn get_todays_quote_from_file() -> String {
    let today = chrono::offset::Local::now().day();
    let quotes_path = match env::var("LOVESAY_PATH") {
        Ok(str) => str.to_string(),
        Err(_) => "~/.config/lovesay/quotes".to_string(),
    };
    let quotes = get_quotes(&quotes_path);

    match quotes.get(today as usize - 1) {
        Some(str) => str.to_string(),
        None => "No quotes for today".to_string(),
    }
}

fn get_quote_vec(today_quote: String) -> Vec<String> {
    let width = match term_size::dimensions() {
        Some((width, _)) => width,
        None => 80,
    };

    if width < 25 {
        return vec![];
    }

    textwrap::wrap(&today_quote, width - 25)
        .iter()
        .map(|line| line.to_string())
        .collect()
}
