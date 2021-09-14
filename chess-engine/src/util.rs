use std::str::FromStr;

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl From<(Position, Position)> for Move {
    fn from((from, to): (Position, Position)) -> Self {
        Self { from, to }
    }
}

impl Move {
    pub fn arabic(s: &str) -> Result<Self, Error> {
        match s.len() {
            4 => Ok(Self {
                from: s[..2].parse()?,
                to: s[2..4].parse().map_err(|err| match err {
                    Error::InvalidNotation { pos, expected } => Error::InvalidNotation {
                        pos: pos + 2,
                        expected,
                    },
                    e => e,
                })?,
            }),
            len @ 5.. => Err(Error::InvalidNotation {
                pos: len - 1,
                expected: "end of string".into(),
            }),
            len @ 0..=3 => Err(Error::InvalidNotation {
                pos: len,
                expected: "more data".into(),
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    file: u8,
    rank: u8,
}

impl Position {
    pub fn file(&self) -> u8 {
        self.file
    }
    pub fn rank(&self) -> u8 {
        self.rank
    }
}

impl From<(u8, u8)> for Position {
    fn from((file, rank): (u8, u8)) -> Self {
        Position { file, rank }
    }
}

impl FromStr for Position {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        if s.len() > 2 {
            return Err(Error::InvalidNotation {
                pos: 3,
                expected: "end of input".into(),
            });
        }
        let file = match s.get(0) {
            c @ Some(b'a'..=b'h') => c.unwrap() - b'a',
            _ => {
                return Err(Error::InvalidNotation {
                    pos: 0,
                    expected: "a to h".into(),
                })
            }
        };
        let rank = match s[1] {
            c @ b'1'..=b'8' => c - b'1',
            _ => {
                return Err(Error::InvalidNotation {
                    pos: 1,
                    expected: "a to h".into(),
                })
            }
        };
        Ok(Self { file, rank })
    }
}
