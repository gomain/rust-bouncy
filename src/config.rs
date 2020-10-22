use std::env::Args;

#[derive(Debug)]
pub struct Config {
    pub frame_width: u32,
    pub frame_height: u32,
}

#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
}

pub struct ParseArgs {
    args: Args,
}

impl Iterator for ParseArgs {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.args.next()
    }
}

impl ParseArgs {
    pub fn new(args: Args) -> ParseArgs {
        ParseArgs { args }
    }

    pub fn get_config(&mut self) -> Result<Config, ParseError> {
        self.config()
    }

    fn config(&mut self) -> Result<Config, ParseError> {
        self.one_arg()?;
        let (frame_width, frame_height) = self.frame_size()?;
        self.no_arg()?;
        Ok(Config {
            frame_width,
            frame_height,
        })
    }

    fn frame_size(&mut self) -> Result<(u32, u32), ParseError> {
        let width = self.one_u32()?;
        let height = self.one_u32()?;
        Ok((width, height))
    }

    fn one_arg(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(s) => Ok(s),
            None => Err(ParseError::TooFewArgs),
        }
    }

    fn no_arg(&mut self) -> Result<(), ParseError> {
        match self.next() {
            Some(_) => Err(ParseError::TooManyArgs),
            None => Ok(()),
        }
    }

    fn one_u32(&mut self) -> Result<u32, ParseError> {
        let s = self.one_arg()?;
        match s.parse() {
            Ok(n) if n > 1 => Ok(n),
            _ => Err(ParseError::InvalidInteger(s)),
        }
    }
}

pub fn parse_args(args: Args) -> Result<Config, ParseError> {
    ParseArgs::new(args).get_config()
}
