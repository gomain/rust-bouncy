use bouncy::{config, game, play};

fn main() {
    let args = std::env::args();
    match config::parse_args(args) {
        Ok(config) => {
            let game = game::Game::new(&config);
            play::Curses::play(game);
        }
        Err(e) => {
            eprintln!("Error parsing arguments: {:?}", e);
        }
    }
}
