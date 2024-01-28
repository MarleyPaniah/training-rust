use rand::Rng;
use std::io;
use std::process::exit;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use unicode_segmentation::UnicodeSegmentation;

fn word_length(word: &String) -> usize {
    return word.graphemes(true).count();
}

fn check_word_length(word: &String, min_length: u16, max_length: u16) -> bool {
    // Return whether a word's length matches conditions.
    if word_length(word) == 0 {
        return false;
    }

    if min_length != max_length {
        if word_length(word) < min_length.into() {
            return false;
        }
        if word_length(word) > max_length.into() {
            return false;
        }
    } else if word_length(word) != min_length.into() {
        return false;
    }

    return true;
}

fn lines_from_file(file_path: impl AsRef<Path>) -> Vec<String> {
    let file: File = File::open(file_path).expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_word_to_guess(file_path: &Path, min_length: u16, max_length: u16) -> String {
    let lines = lines_from_file(file_path);

    let mut word: String;
    loop {
        let index = rand::thread_rng().gen_range(0..lines.len()); //infered from the size of the file
        word = lines[index].clone(); // Had to "copy" the value the string into the function's owned variable

        if !check_word_length(&word, min_length, max_length) {
            continue;
        }
        break;
    }

    // println!("DEBUG: Word to guess: {word}.");
    return word;
}

fn read_input() -> String {
    let mut guess: String;

    'user_input: loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failure to read letter.");

        guess = input.trim().to_string();

        if !check_word_length(&guess, 1, 1) {
            eprintln!("Guess must be exactly (1) character!");
            continue;
        }

        break 'user_input;
    }

    return guess;
}

fn character_is_in_word(word_to_guess: &String, guess: &String) -> bool {
    return word_to_guess.contains(guess);
}

fn reveal_guessed_letters(word_to_guess: &String, guessed_letters: &Vec<String>) -> String {
    /*
     Return a String with the revealed letters
     and the non yet guessed ones replaced by "_".
    */
    let mut letters_revealed: String = String::new();
    for s in word_to_guess.graphemes(true) {
        if guessed_letters.contains(&s.to_string()) {
            letters_revealed += s;
        } else {
            letters_revealed += "_";
        }
        letters_revealed += " " // space to improve display
    }

    return letters_revealed;
}

fn main() {
    let file_path: &Path = Path::new("./src/english_words.txt");
    let min_length: u16 = 5;
    let max_length: u16 = 1_000; // Eqv to "no max size"
    let max_errors: u8 = 10;

    let word_to_guess: String = get_word_to_guess(file_path, min_length, max_length);

    let mut guessed_letters: Vec<String> = Vec::new();
    let mut nb_errors: u8 = 0;

    while nb_errors < max_errors {
        let reveal_string: String = reveal_guessed_letters(&word_to_guess, &guessed_letters);

        if !reveal_string.contains("_") {
            println!("SUCCESS -- You found it! The word was '{}'.", word_to_guess);
            exit(0);
        }

        println!(
            "Guess the word: {} -- (len: {}) (errors: {}/{})",
            reveal_string,
            word_length(&word_to_guess),
            nb_errors,
            max_errors
        );

        let guess: String = read_input();

        if !character_is_in_word(&word_to_guess, &guess) {
            println!("{guess}: Incorrect letter.");
            nb_errors += 1;
            continue;
        }

        if !guessed_letters.contains(&guess) {
            println!("{guess}: Letter found.");
            guessed_letters.push(guess);
        }
    }

    println!(
        "FAILED -- Too many errors ({max_errors}). The word was '{}'.",
        word_to_guess
    )
}

#[test]
fn test_check_length() {
    assert_eq!(check_word_length(&String::from("a"), 1, 1), true);
    assert_eq!(check_word_length(&String::from("babeloued"), 9, 9), true);
}
