use super::game;

pub struct Curses {
    window: pancurses::Window,
    game: game::Game,
}

impl Curses {
    pub fn new(window: pancurses::Window, game: game::Game) -> Curses {
        Curses { window, game }
    }

    fn run(&mut self) -> ! {
        loop {
            self.draw();
            self.game.step();
            std::thread::sleep(std::time::Duration::from_millis(33));
        }
    }

    pub fn play(window: pancurses::Window, game: game::Game) {
        Self::new(window, game).run();
    }

    fn draw(&self) {
        let Curses { window, game } = self;
        window.clear();
        window.printw(game.to_string());
        window.refresh();
    }
}
