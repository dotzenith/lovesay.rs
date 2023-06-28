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

    // Hearts
    let oneheart: String = kolor.red(heart);
    let twoheart: String = kolor.purple(heart);
    let threeheart: String = kolor.blue(heart);
    let fourheart: String = kolor.green(heart);
    let fiveheart: String = kolor.orange(heart);
    let sixheart: String = kolor.yellow(heart);

    // Quote lines
    let mut printable_quotes: Vec<String> = vec![String::new(); 5];
    for (i, quote) in quote_vec.iter().enumerate() {
        if i < 5 {
            printable_quotes[i] = format!("{} {} {}", oneheart, kolor.white(quote), oneheart);
        } else {
            printable_quotes.push(format!("{} {} {}", oneheart, kolor.white(quote), oneheart));
        }
    }

    // Heart
    println!("   {oneheart} {oneheart}   {oneheart} {oneheart}   ");
    println!(" {twoheart}     {twoheart}     {twoheart}      {}", printable_quotes[0]);
    println!(" {threeheart}           {threeheart}      {}", printable_quotes[1]);
    println!("   {fourheart}       {fourheart}        {}", printable_quotes[2]);
    println!("     {fiveheart}   {fiveheart}          {}", printable_quotes[3]);
    println!("       {sixheart}            {}", printable_quotes[4]);

    for quote in quote_vec.iter().skip(5) {
        println!("                    {} {} {}", oneheart, kolor.white(quote), oneheart);
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
    let mut width: usize = match term_size::dimensions() {
        Some((width, _)) => width,
        None => 80,
    };

    match env::var("LOVESAY_MAX_WIDTH") {
        Ok(w) => width = w.parse().unwrap_or(80),
        Err(_) => (),
    }

    if width < 25 {
        return vec![];
    }

    textwrap::wrap(&today_quote, width - 25)
        .iter()
        .map(|line| line.to_string())
        .collect()
}
