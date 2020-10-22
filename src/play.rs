use super::game;

pub struct Curses {
    window: pancurses::Window,
    border: pancurses::Window,
    frame: pancurses::Window,
    game: game::Game,
}

impl Curses {
    pub fn new(window: pancurses::Window, game: game::Game) -> Curses {
        let game::Frame { width, height } = game.frame;
        let width = width as i32;
        let height = height as i32;
        let border = window.derwin(height + 2, width + 2, 0,0).unwrap();
        let frame = border.derwin(height, width,1,1).unwrap();
        Curses {
            window,
            border,
            frame,
            game,
        }
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
        let Curses { window, border, frame, game } = self;
        window.clear();
        border.border('|','|','-','-','+','+','+','+');
        border.touch();
        frame.mvaddch(game.ball.y as i32,game.ball.x as i32,'O');
        frame.touch();
        window.touch();
        window.refresh();
    }

        
}
