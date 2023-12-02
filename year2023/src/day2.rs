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

#[test]
fn solve() {
    println!("part I : {}\n", possible_games().unwrap(),);
}
