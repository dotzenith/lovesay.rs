use chrono::Datelike;
use clap::{arg, command, ArgMatches};
use shellexpand;
use std::io::{self, IsTerminal};
use std::{env, fs};
use term_size;
use textwrap;

mod printers;

fn main() {
    let heart = match env::var("LOVESAY_NO_NERD") {
        Ok(_) => "\u{2665}",
        Err(_) => "\u{f004}",
    };

    let matches = command!()
        .arg(arg!([message]).trailing_var_arg(true).num_args(..))
        .get_matches();

    let today_quote = get_quote(matches);
    let quote_vec = get_quote_vec(today_quote);

    match env::var("LOVESAY_COLORSCHEME") {
        Ok(str) => printers::print_with_kolorz(&str, heart, &quote_vec),
        Err(_) => printers::print_with_colored(heart, &quote_vec),
    };
}

fn get_quote(matches: ArgMatches) -> String {
    if io::stdin().is_terminal() {
        match matches.get_many::<String>("message") {
            Some(messages) => messages
                .map(|val| val.to_owned())
                .collect::<Vec<_>>()
                .join(" "),
            None => get_todays_quote_from_file(),
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
