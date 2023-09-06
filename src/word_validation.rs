use super::game_logic::Game;

pub struct Word {
    word: String,
}

impl Word {
    fn new(word: String) -> Self {
        Word {
            word: word.to_uppercase()
        }
    }

    fn get_contained_letters_position(&self, game: &Game) -> Vec<bool> {
        let mut contains_letters: Vec<bool> = Vec::new();
        for letter in self.word.chars() {
            if game.get_word().contains(letter) {
                contains_letters.push(true);
            }else{
                contains_letters.push(false);
            }
        }
        contains_letters
    }

    fn get_correct_letters_position(&self, game: &Game) -> Vec<bool> {
        let mut correct_letters: Vec<bool> = Vec::new();
        for (char_game_word, char_input_word) in game.get_word().chars().zip(self.word.chars()) {
            if char_game_word == char_input_word {
                correct_letters.push(true);
            }else{
                correct_letters.push(false);
            }
        }
        correct_letters
    }

    fn is_valid(&self, game: &Game) -> bool {
        Game::has_same_length(game, &self) && self.word.to_uppercase() == self.word
    }

    pub fn get_length(&self) -> usize {
        self.word.len()
    }

}

#[test]
fn test_is_valid_r_t () {
    let game = Game::new();
    let word = Word::new(String::from("ASDF"));
    let word_2 = Word::new(String::from("asDf"));
    assert!(word.is_valid(&game));
    assert_eq!(word.word, String::from("ASDF"));
    assert_ne!(word.word, String::from("asdf"));
    assert!(&word.word.to_uppercase() == &word.word);
    assert!(word_2.is_valid(&game));
}

#[test]
fn test_is_valid_r_f () {
    let game = Game::new();
    let word = Word::new(String::from("ASDFG"));
    assert!(!word.is_valid(&game));
}

#[test]
fn test_get_contained_letters_position_r_t () {
    let game = Game::new();
    let word = Word::new(String::from("EETA"));
    assert_eq!(word.get_contained_letters_position(&game), vec![true,true,true,false]);
}

#[test]
fn test_get_correct_letters_position_r_t () {
    let game = Game::new();
    let word = Word::new(String::from("EETA"));
    let word_2 = Word::new(String::from("TEST"));
    assert_eq!(word.get_correct_letters_position(&game), vec![false,true,false,false]);
    assert_eq!(word_2.get_correct_letters_position(&game), vec![true,true,true,true]);
}
