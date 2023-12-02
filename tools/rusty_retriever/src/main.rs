use std::{
    fs,
    io::{self, Write},
};

use curl::easy::Easy;

enum Input {
    Year,
    Day,
}

impl Into<&[u8]> for Input {
    fn into(self) -> &'static [u8] {
        match self {
            Input::Year => b"Year: ",
            Input::Day => b"Day: ",
        }
    }
}

fn main() {
    let (mut output_handle, input_handle) = (io::stdout(), io::stdin());
    let mut input_helper = |input: Input| -> u16 {
        let mut user_input = String::new();
        let (_, _, _) = (
            output_handle.write_all(input.into()),
            output_handle.flush(),
            input_handle.read_line(&mut user_input),
        );
        user_input.trim().parse().unwrap()
    };

    // UI
    // Ask the user for the year and day from which they want to get the puzzle input
    let (year, day) = (input_helper(Input::Year), input_helper(Input::Day));
    let _ = std::fs::create_dir_all(format!("year{year}/input/"));
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("year{year}/input/day{day}.txt"))
        .unwrap();

    // cURL handle for retrieval
    let mut easy = Easy::new();
    // Path to the netscape cookies format file
    easy.cookie_file("COOKIES.txt").unwrap();
    easy.url(format!("https://adventofcode.com/{year}/day/{day}/input").as_str())
        .unwrap();
    easy.write_function(move |data| {
        let _ = file.write_all(data);
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}
