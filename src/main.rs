use clap::Parser;
use colored::Colorize;
use rand::seq::IndexedRandom;

const DEFAULT_SYMBOLS: &'static str = "!@#$%_-";
const DEFAULT_SEPARATOR: &str = "-";
const GROUP_SIZE: usize = 4;

/// Generates a password. By default, it uses a combination of uppercase letters, lowercase letters, and digits.
#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Length of password
    #[arg(short = 'n', long, default_value_t = 16)]
    length: usize,

    /// Do not use lowercase letters
    #[arg(short = 'l')]
    without_lowercase: bool,

    /// Do not use uppercase letters
    #[arg(short = 'u')]
    without_uppercase: bool,

    /// Do not use digits
    #[arg(short = 'd')]
    without_digits: bool,

    /// Use additional characters (If not specified, it defaults to "!@#$%_-")
    #[arg(short = 'a')]
    additional_characters: Option<Option<String>>,

    /// Specify custom character set
    #[arg(short = 'c')]
    custom_characters: Option<String>,

    /// Separate into groups of 4 (If not specified, it defaults to '-')
    #[arg(short = 'x')]
    separator: Option<Option<String>>,
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
        characters.push_str(&additional.unwrap_or(DEFAULT_SYMBOLS.to_string()));
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

    // separate
    let output = if let Some(separator) = parser.separator {
        separate(
            &chosen,
            GROUP_SIZE,
            &separator.unwrap_or(DEFAULT_SEPARATOR.to_string()),
        )
    } else {
        chosen
    };

    println!("{output}");
}

/// select n randomly from the given string with duplicates
fn choose_ramdomly(characters: &str, n: usize) -> String {
    let mut rng = rand::rng();

    let chars: Vec<char> = characters.chars().collect();

    (0..n).filter_map(|_| chars.choose(&mut rng)).collect()
}

/// separate by separator symbol
fn separate(input: &str, group_size: usize, separator: &str) -> String {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(group_size)
        .map(|ch| ch.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(separator)
}
