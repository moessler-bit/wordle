use rand::seq::SliceRandom;

use super::word_validation::Word;

static WORDS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/wordle_lang.txt"));

pub struct Game {
    secret_word: String,
    max_tries: u8,
    current_try: u8,
    won: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            secret_word: generate_word(),
            max_tries: 5,
            current_try: 0,
            won: false,
        }
    }

    pub fn try_word(&mut self, word: &Word) -> (Vec<bool>, Vec<bool>) {
        self.add_try();
        self.check_win(word);

        return (
            self.get_contained_letters_position(word),
            self.get_correct_letters_position(word),
        );
    }

    pub fn is_allowed_trying(&self) -> bool {
        !self.won && self.tries_left() > 0
    }

    pub fn has_same_length(&self, word: &Word) -> bool {
        self.secret_word.len() == word.word.len()
    }

    fn correct_word(&self, word: &Word) -> bool {
        self.secret_word == word.word
    }

    fn check_win(&mut self, word: &Word) {
        if self.is_win(word) {
            self.won = true
        };
    }

    fn is_win(&self, word: &Word) -> bool {
        self.correct_word(word) && self.tries_left() > 0
    }

    pub fn did_win(&self) -> bool {
        self.won
    }

    fn add_try(&mut self) {
        self.current_try = self.current_try + 1;
    }

    fn tries_left(&self) -> u8 {
        self.max_tries - self.current_try
    }

    pub fn get_length(&self) -> usize {
        self.secret_word.len()
    }

    fn get_contained_letters_position(&self, word: &Word) -> Vec<bool> {
        let mut contains_letters: Vec<bool> = Vec::new();
        for letter in word.word.chars() {
            if self.secret_word.contains(letter) {
                contains_letters.push(true);
            } else {
                contains_letters.push(false);
            }
        }
        // println!("{:#?} contains", contains_letters);
        contains_letters
    }

    fn get_correct_letters_position(&self, word: &Word) -> Vec<bool> {
        let mut correct_letters: Vec<bool> = Vec::new();
        for (char_game_word, char_input_word) in self.secret_word.chars().zip(word.word.chars()) {
            if char_game_word == char_input_word {
                correct_letters.push(true);
            } else {
                correct_letters.push(false);
            }
        }
        // println!("{:#?} correct", correct_letters);
        correct_letters
    }
}

fn generate_word() -> String {
    let words: Vec<&str> = WORDS.trim().split('\n').collect();
    let mut rng = rand::thread_rng();
    let word = words.choose(&mut rng).unwrap().to_string();
    // println!("{} <- secret word", word);
    word.to_uppercase()
}
