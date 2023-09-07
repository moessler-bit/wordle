use super::game_logic::Game;

pub struct Word {
    pub word: String,
}

impl Word {
    pub fn new(word: String) -> Self {
        Word {
            word: word.to_uppercase(),
        }
    }

    pub fn is_valid(&self, game: &Game) -> bool {
        Game::has_same_length(game, &self) && self.word.to_uppercase() == self.word
    }
}
