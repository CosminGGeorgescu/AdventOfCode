use std::collections::HashMap;

use crate::input;

fn possible_games() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(input::DAY2)?
        .trim_end()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .map(|(id, &game)| {
            let mut sets = game
                .split(": ")
                .nth(1)
                .unwrap()
                .split(&[';', ' ', ','])
                .filter(|&x| x != "");

            if true
                && loop {
                    if let Some(nr) = sets.next() {
                        let (nr, color) = (nr.parse::<usize>().unwrap(), sets.next().unwrap());
                        if (color == "red" && nr > 12)
                            || (color == "green" && nr > 13)
                            || (color == "blue" && nr > 14)
                        {
                            break false;
                        }
                    } else {
                        break true;
                    }
                }
            {
                id + 1
            } else {
                0
            }
        })
        .sum())
}

fn min_set_power_sum() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(input::DAY2)?
        .trim_end()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&game| {
            let mut sets = game
                .split(": ")
                .nth(1)
                .unwrap()
                .split(&[';', ' ', ','])
                .filter(|&x| x != "");

            let mut hm = HashMap::<&str, usize>::new();
            hm.insert("red", 0);
            hm.insert("green", 0);
            hm.insert("blue", 0);

            while let Some(nr) = sets.next() {
                let (nr, color) = (nr.parse::<usize>().unwrap(), sets.next().unwrap());
                if nr > hm[color] {
                    hm.insert(color, nr);
                }
            }

            hm["red"] * hm["green"] * hm["blue"]
        })
        .sum())
}

#[test]
fn solve() {
    println!(
        "part I : {}\npart II: {}\n",
        possible_games().unwrap(),
        min_set_power_sum().unwrap()
    );
}
