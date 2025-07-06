use clap::{CommandFactory, Parser};
use colored::Colorize;
use rand::seq::IndexedRandom;

/// Generates a password. By default, it uses a combination of uppercase letters, lowercase letters, and digits.
#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// length of password
    #[arg(short = 'n', long, default_value_t = 16)]
    length: usize,

    /// do not use uppercase letters
    #[arg(short = 'l')]
    without_lowercase: bool,

    /// do not use lowercase letters
    #[arg(short = 'u')]
    without_uppercase: bool,

    /// do not use digits
    #[arg(short = 'd')]
    without_digits: bool,

    /// use additional characters to the default set
    #[arg(short = 'a')]
    additional_characters: Option<String>,

    /// specify custom character set
    #[arg(short = 'c')]
    custom_characters: Option<String>,
}

fn main() {
    let parser = Args::parse();

    let mut characters = String::new();

    if !parser.without_lowercase {
        characters.extend('a'..='z');
    }

    if !parser.without_uppercase {
        characters.extend('A'..='Z');
    }

    if !parser.without_digits {
        characters.extend('0'..='9');
    }

    if let Some(additional) = parser.additional_characters {
        characters += &additional;
    }

    // replace characters
    if let Some(custom) = parser.custom_characters {
        characters = custom;
    }

    if characters.is_empty() {
        eprintln!("{}", "The character set cannot be empty.".red().bold());
        std::process::exit(1);
    }

    // choose randomly
    let chosen = choose_ramdomly(&characters, parser.length);

    println!("{chosen}");
}

/// select n randomly from the given string with duplicates
fn choose_ramdomly(characters: &str, n: usize) -> String {
    let mut rng = rand::rng();

    let chars: Vec<char> = characters.chars().collect();

    (0..n).map(|_| chars.choose(&mut rng)).flatten().collect()
}
