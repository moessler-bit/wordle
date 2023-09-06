use super::word_validation::Word;
//use word_generator::{langs, *};
use std::{fs::File, io::BufReader, env};

pub struct Game {
    secret_word: String,
    max_tries: u8,
    current_try: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            secret_word: generate_word(),
            max_tries: 5,
            current_try: 0,
        }
    }

    // returns tries left
    pub fn try_word(&mut self, word: Word) -> u8 {
        self.current_try = self.current_try + 1;
        // tries left
        self.max_tries - self.current_try
    }

    fn get_length(&self) -> usize {
        self.secret_word.len()
    }

    pub fn has_same_length(&self, word: &Word) -> bool {
        self.get_length() == word.get_length()
    }

    pub fn get_word(&self) -> &String {
        &self.secret_word
    }

}
fn generate_word() -> String {
    static WORDS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/wordle_lang.txt"));
    println!("{}",WORDS);
    let reader = BufReader::new(WORDS);
    generate_words(reader, 3, 1).unwrap()[0].to_owned()
}
