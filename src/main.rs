use wordle::game_logic;
use wordle::input;
use wordle::output;

fn main() {
    // let x = input::read_input();
    // output::print(&x);
    let game = game_logic::Game::new();
    println!("{}", game.get_word());
}
