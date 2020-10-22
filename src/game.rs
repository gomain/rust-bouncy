use super::config;

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

pub struct Ball {
    pub x: u32,
    pub y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }
        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
}

pub struct Game {
    pub frame: Frame,
    pub ball: Ball,
}

impl Game {
    pub fn new(config: &config::Config) -> Game {
        let frame = Frame {
            width: config.frame_width,
            height: config.frame_height,
        };
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };
        Game { frame, ball }
    }

    pub fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

use std::fmt::{Display, Formatter};

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        use std::convert::TryFrom;
        let width: usize = usize::try_from(self.frame.width).unwrap();
        let top = |fmt: &mut Formatter| writeln!(fmt, "+{}+", "--".repeat(width));
        let bottom = top;
        let inbetweens = |fmt: &mut Formatter, col: Option<u32>| {
            let the_ball = vec![0xF0, 0x9F, 0x8E, 0x83];
            let pre_ball = "  ".repeat(usize::try_from(col.unwrap_or(0)).unwrap());
            let ball = if col.is_some() {
                std::str::from_utf8(&the_ball).expect("get &str from vec<u8>")
            } else {
                "  "
            };
            let post_ball =
                "  ".repeat(usize::try_from(self.frame.width - col.unwrap_or(0) - 1).unwrap());
            writeln!(fmt, "|{}{}{}|", pre_ball, ball, post_ball)
        };
        top(fmt)?;
        for row in 0..self.frame.height {
            inbetweens(
                fmt,
                if row == self.ball.y {
                    Some(self.ball.x)
                } else {
                    None
                },
            )?;
        }
        bottom(fmt)
    }
}
