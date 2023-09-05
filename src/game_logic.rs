struct Game {
    secret_word: String,
    max_tries: u8,
    current_try: u8
}

impl Game {
    fn new() -> Self {
        Game { secret_word: String::from("Test"), max_tries: 5, current_try: 0 }
    }

    // returns tries left
    fn try_word(word: String) -> () {
    }
}

struct Word {
    
}
