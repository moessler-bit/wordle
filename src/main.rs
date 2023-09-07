use wordle::game_logic::{Game, *};
use wordle::output::print;
use wordle::{input, word_validation};

fn main() {
    let mut game = Game::new();
    println!(
        "{}",
        "$ -> correct position\n# -> is contained in the word\n_ -> is not contained in the word"
    );
    println!("\n{}: {}", "Guess a word of length", game.get_length());
    while !game.did_win() && game.is_allowed_trying() {
        let input = input::read_input();
        let word = word_validation::Word::new(input);
        if word.is_valid(&game) && game.is_allowed_trying() {
            let (contained, correct): (Vec<bool>, Vec<bool>) = game.try_word(&word);
            print(&word.word, contained, correct);
        }
    }
    match game.did_win() {
        true => println!("{}", "You won!"),
        false => println!("{}", "You lost!"),
    }
}
