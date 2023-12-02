use std::fmt::Display;

use crate::input;

#[derive(Debug)]
pub enum Either {
    Left(usize),
    Right(isize),
}

impl Display for Either {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(ans) => write!(f, "{}", ans),
            Self::Right(_) => write!(f, "{:?}", self),
        }
    }
}

pub fn floor() -> Result<isize, std::io::Error> {
    Ok(std::fs::read(input::DAY1)?
        .iter()
        .fold(0, |acc, &x| acc + if x == b'(' { 1 } else { -1 }))
}

pub fn basement_index() -> Result<Either, std::io::Error> {
    Ok(std::fs::read(input::DAY1)?
        .iter()
        .enumerate()
        .fold(Either::Right(0), |acc, (i, &x)| {
            if let Either::Right(sum) = acc {
                let floor = sum + if x == 40 { 1 } else { -1 };
                match floor {
                    -1 => Either::Left(i + 1),
                    _ => Either::Right(floor),
                }
            } else {
                acc
            }
        }))
}

#[test]
fn solve() {
    println!(
        "part I : {}\npart II: {}",
        floor().unwrap(),
        basement_index().unwrap()
    );
}
