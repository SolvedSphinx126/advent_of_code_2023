use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let lines = read_lines("input.dat").unwrap();
    let sum: u32 = lines.into_iter()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .map(|line| line.chars().into_iter()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>())
            .map(|mut line| {
                let mut str = String::new();
                str.push(line.chars().nth(0).unwrap());
                str.push(line.pop().unwrap());
                str
            })
        .map(|line| line.parse::<u32>())
        .filter(|num| num.is_ok())
        .map(|num| num.unwrap())
        .sum();
    println!("{}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
