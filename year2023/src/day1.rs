use aho_corasick::{AhoCorasick, Match};

use crate::input;

pub fn compute_calibration_values_sum() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(input::DAY1)?
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

pub fn real_calibration_values_sum() -> Result<usize, impl std::error::Error> {
    fn convert_literal_to_digit(literal: &str) -> char {
        match literal {
            "one" | "1" => '1',
            "two" | "2" => '2',
            "three" | "3" => '3',
            "four" | "4" => '4',
            "five" | "5" => '5',
            "six" | "6" => '6',
            "seven" | "7" => '7',
            "eight" | "8" => '8',
            "nine" | "9" => '9',
            _ => '\0',
        }
    }

    AhoCorasick::builder()
        .ascii_case_insensitive(false)
        .build(&[
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ])
        .map(|ac| -> Result<usize, std::io::Error> {
            Ok(std::fs::read_to_string(input::DAY1)?
                .split_terminator('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|&line| {
                    let matches: Vec<Match> = ac.find_overlapping_iter(line).collect();
                    let first = matches
                        .first()
                        .map(|&n| convert_literal_to_digit(&line[n.start()..n.end()]));
                    let last = matches
                        .last()
                        .map(|&n| convert_literal_to_digit(&line[n.start()..n.end()]))
                        .or(first);
                    first
                        .zip(last)
                        .map(|(chr1, chr2)| format!("{chr1}{chr2}").parse::<usize>().unwrap())
                        .unwrap()
                })
                .sum())
        })
        .unwrap()
}

#[test]
fn solve() {
    println!(
        "part I : {}\npart II: {}",
        compute_calibration_values_sum().unwrap(),
        real_calibration_values_sum().unwrap()
    );
}
