use atty::Stream;
use chrono::Datelike;
use shellexpand;
use std::{env, fs, io};
use term_size;
use textwrap;

fn main() {
    let mut heart = "\u{f004}";

    let mut argv: Vec<String> = env::args().map(|v| v.to_owned()).collect();
    if argv.len() > 1 && argv.get(1).unwrap() == "--no-nerd" {
        heart = "\u{2665}";
        argv.remove(1);
    }

    let today_quote = get_todays_quote(argv);
    let quote_vec = get_quote_vec(today_quote);

    // Hearts
    let oneheart: String = format!("[38;2;243;139;168m{heart}[0m");
    let twoheart: String = format!("[38;2;203;166;247m{heart}[0m");
    let threeheart: String = format!("[38;2;137;180;250m{heart}[0m");
    let fourheart: String = format!("[38;2;166;227;161m{heart}[0m");
    let fiveheart: String = format!("[38;2;250;179;135m{heart}[0m");
    let sixheart: String = format!("[38;2;249;226;175m{heart}[0m");

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

fn get_todays_quote(argv: Vec<String>) -> String {
    if atty::is(Stream::Stdin) {
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

    let quotes: Vec<String> = fs::read_to_string(shellexpand::tilde(&quotes_path).to_string())
        .unwrap_or("No quotes file found".to_string())
        .lines()
        .map(|str| str.to_string())
        .collect();

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
