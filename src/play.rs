use super::game;

pub struct Curses {
    window: pancurses::Window,
    border: pancurses::Window,
    frame: pancurses::Window,
    game: game::Game,
    ball_xy: Option<(u32, u32)>,
}

impl Drop for Curses {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl Curses {
    fn new(game: game::Game) -> Curses {
        let window = pancurses::initscr();
        let game::Frame { width, height } = game.frame;
        let width = width as i32;
        let height = height as i32;
        let border = window.derwin(height + 2, width + 2, 0, 0).unwrap();
        let frame = border.derwin(height, width, 1, 1).unwrap();
        Curses {
            window,
            border,
            frame,
            game,
            ball_xy: None,
        }
    }

    fn run(&mut self) {
        self.draw_all();
        self.window.timeout(8);
        while !self.game.ended() {
            self.game.step();
            self.draw_ball();
            self.refresh();
            if let Some(pancurses::Input::Character('q')) = self.window.getch() {
                self.game.end();
            }
        }
    }

    pub fn play(game: game::Game) {
        Self::new(game).run();
    }

    fn draw_all(&mut self) {
        self.draw_border();
        self.draw_ball();
        self.refresh();
    }

    fn draw_border(&self) {
        let border = &self.border;
        border.border(
            pancurses::ACS_VLINE(),
            pancurses::ACS_VLINE(),
            pancurses::ACS_HLINE(),
            pancurses::ACS_HLINE(),
            pancurses::ACS_ULCORNER(),
            pancurses::ACS_URCORNER(),
            pancurses::ACS_LLCORNER(),
            pancurses::ACS_LRCORNER(),
        );
    }
    fn draw_ball(&mut self) {
        let Curses {
            frame,
            game,
            ball_xy,
            ..
        } = self;
        if let Some((x, y)) = ball_xy {
            frame.color_set(pancurses::COLOR_RED);
            frame.mvaddch(*y as i32, *x as i32, 'O');
        }
        let (x, y) = game.ball.get_xy();
        frame.color_set(-1);
        frame.mvaddch(y as i32, x as i32, 'O');
        self.ball_xy = Some((x, y));
    }
    fn refresh(&self) {
        let window = &self.window;
        window.touch();
        window.refresh();
    }
}
