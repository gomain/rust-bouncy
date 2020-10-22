use bouncy::{config, game, play};

fn main() {
    let args = std::env::args();
    match config::parse_args(args) {
        Ok(config) => {
            let game = game::Game::new(&config);
            let window = pancurses::initscr();
            play::Curses::play(window, game);
        }
        Err(e) => {
            eprintln!("Error parsing arguments: {:?}", e);
        }
    }
}
