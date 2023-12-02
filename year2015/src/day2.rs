use crate::input;

pub fn wrapping_paper_area() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(input::DAY2)?
        .split_terminator('\n')
        .fold(0, |acc, gift| {
            let mut dimensions: Vec<usize> = gift
                .split('x')
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            dimensions.sort();
            acc + 2
                * (dimensions[0] * dimensions[1]
                    + dimensions[0] * dimensions[2]
                    + dimensions[1] * dimensions[2])
                + dimensions[0] * dimensions[1]
        }))
}

pub fn ribbon_length() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(input::DAY2)?
        .split_terminator('\n')
        .fold(0, |acc, gift| {
            let mut dimensions: Vec<usize> = gift
                .split('x')
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            dimensions.sort();
            acc + 2 * (dimensions[0] + dimensions[1]) + dimensions.iter().product::<usize>()
        }))
}

#[test]
fn solve() {
    println!(
        "part I : {}\npart II: {}",
        wrapping_paper_area().unwrap(),
        ribbon_length().unwrap()
    );
}
