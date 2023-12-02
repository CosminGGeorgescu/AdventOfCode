use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use crate::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point(isize, isize);

impl Point {
    fn advance(self, direction: u8) -> Self {
        self + match direction {
            b'^' => Point(0, 1),
            b'v' => Point(0, -1),
            b'>' => Point(1, 0),
            b'<' => Point(-1, 0),
            _ => panic!("Check the contents of your input file"),
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn houses_visited() -> Result<usize, std::io::Error> {
    let mut houses_visited: HashMap<Point, usize> = HashMap::new();
    std::fs::read(input::DAY3)?
        .iter()
        .fold(Point::default(), |position, &direction| {
            houses_visited
                .entry(position)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            position.advance(direction)
        });
    Ok(houses_visited.len())
}

fn houses_visited_with_robosanta() -> Result<usize, std::io::Error> {
    let mut houses_visited_by_santa: HashMap<Point, usize> = HashMap::new();
    let mut houses_visited_by_robosanta: HashMap<Point, usize> = HashMap::new();
    let _ = std::fs::read(input::DAY3)?.iter().enumerate().fold(
        (Point::default(), Point::default()),
        |(santa_position, robosanta_position), (i, &direction)| match i % 2 == 0 {
            true => {
                houses_visited_by_santa
                    .entry(santa_position)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                (santa_position.advance(direction), robosanta_position)
            }
            false => {
                houses_visited_by_robosanta
                    .entry(robosanta_position)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                (santa_position, robosanta_position.advance(direction))
            }
        },
    );
    Ok(houses_visited_by_santa
        .keys()
        .cloned()
        .collect::<HashSet<Point>>()
        .union(&houses_visited_by_robosanta.keys().cloned().collect())
        .count())
}

#[test]
fn solve() {
    println!(
        "part I : {}\npart II: {}",
        houses_visited().unwrap(),
        houses_visited_with_robosanta().unwrap()
    );
}
