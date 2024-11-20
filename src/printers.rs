use colored::Colorize;
use kolorz::Kolor;
use std::fmt::Display;

pub fn print_with_kolorz(kolor: &str, heart: &str, quotes: &Vec<String>) -> () {
    let kolor = Kolor::new(kolor);

    let hearts = (
        kolor.red(heart),
        kolor.purple(heart),
        kolor.blue(heart),
        kolor.green(heart),
        kolor.orange(heart),
        kolor.yellow(heart),
    );

    let text_quotes: Vec<_> = quotes.iter().map(|item| kolor.text(item)).collect();

    common_printer(hearts, &text_quotes);
}

pub fn print_with_colored(heart: &str, quotes: &Vec<String>) -> () {
    let hearts = (
        heart.red(),
        heart.purple(),
        heart.cyan(),
        heart.blue(),
        heart.green(),
        heart.yellow(),
    );

    common_printer(hearts, quotes);
}

fn common_printer<T: Display, U: Display>(hearts: (T, T, T, T, T, T), quotes: &Vec<U>) {
    let mut printable_quotes: Vec<String> = vec![String::new(); 5];
    for (i, quote) in quotes.iter().enumerate() {
        if i < 5 {
            printable_quotes[i] = format!("{} {} {}", hearts.0, quote, hearts.0);
        } else {
            printable_quotes.push(format!("{} {} {}", hearts.0, quote, hearts.0));
        }
    }

    println!("   {} {}   {} {}   ", hearts.0, hearts.0, hearts.0, hearts.0);
    println!(" {}     {}     {}      {}", hearts.1, hearts.1, hearts.1, printable_quotes[0]);
    println!(" {}           {}      {}", hearts.2, hearts.2, printable_quotes[1]);
    println!("   {}       {}        {}", hearts.3, hearts.3, printable_quotes[2]);
    println!("     {}   {}          {}", hearts.4, hearts.4, printable_quotes[3]);
    println!("       {}            {}", hearts.5, printable_quotes[4]);

    for quote in quotes.iter().skip(5) {
        println!("                    {} {} {}", hearts.0, quote, hearts.0);
    }
}
