pub enum Either {
    Left(usize),
    Right(isize),
}

pub fn floor() -> Result<isize, std::io::Error> {
    Ok(std::fs::read("day1.txt")?
        .iter()
        .fold(0, |acc, &x| acc + if x == b'(' { 1 } else { -1 }))
}

pub fn basement_index() -> Result<Either, std::io::Error> {
    Ok(std::fs::read("day1.txt")?
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
