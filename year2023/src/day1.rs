use regex::{Match, Regex};

pub fn compute_calibration_values_sum() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string("src/input/day1.txt")?
        .split_terminator('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| {
            let mut digit_it = line.chars().filter(|d| d.is_ascii_digit());
            let first = digit_it.nth(0);
            let last = digit_it.nth_back(0).or(first);
            first
                .zip(last)
                .map(|(chr1, chr2)| format!("{chr1}{chr2}").parse::<usize>().unwrap())
                .unwrap()
        })
        .sum())
}

///! FIX OVERLAPPING MATCHING

pub fn idk() -> Result<usize, std::io::Error> {
    fn convert_literal_to_digit(literal: &str) -> Option<char> {
        match literal {
            "one" => Some('1'),
            "two" => Some('2'),
            "three" => Some('3'),
            "four" => Some('4'),
            "five" => Some('5'),
            "six" => Some('6'),
            "seven" => Some('7'),
            "eight" => Some('8'),
            "nine" => Some('9'),
            "1" => Some('1'),
            "2" => Some('2'),
            "3" => Some('3'),
            "4" => Some('4'),
            "5" => Some('5'),
            "6" => Some('6'),
            "7" => Some('7'),
            "8" => Some('8'),
            "9" => Some('9'),
            _ => None,
        }
    }

    Ok(std::fs::read_to_string("src/input/day1.txt")?
        .split_terminator('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&line| {
            let pattern = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
            let matches: Vec<Match> = pattern.find_iter(line).collect();
            let first = matches
                .first()
                .map(|&n| convert_literal_to_digit(&line[n.start()..n.end()]).unwrap());
            let last = matches
                .last()
                .map(|&n| convert_literal_to_digit(&line[n.start()..n.end()]).unwrap())
                .or(first);
            println!("{:?} / {:?}", first, last);
            first
                .zip(last)
                .map(|(chr1, chr2)| format!("{chr1}{chr2}").parse::<usize>().unwrap())
                .unwrap()
        })
        .sum())
}
