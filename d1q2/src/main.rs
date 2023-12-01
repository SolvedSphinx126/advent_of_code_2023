use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let nums = HashMap::from([
        ("zeero", "0"),
        ("onne", "1"),
        ("two", "2"),
        ("three", "3"),
        ("foour", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seeveen", "7"),
        ("eight", "8"),
        ("ninne", "9"),
    ]);
    let lines = read_lines("input.dat").unwrap();
    let sum: u32 = lines
        .into_iter()
        .map_while(Result::ok)
        // duplicate every letter that is both the first and last letter of a digit spelled out to prevent overlaps
        // dictionary must be adjusted for internal occurances of those words
        .map(|mut line| {
            line = line.replace('e', "ee");
            line = line.replace('o', "oo");
            line = line.replace('t', "tt");
            line = line.replace('n', "nn");
            line
        })
        .map(|mut line| {
            for num in nums.iter() {
                line = line.replace(num.0, num.1);
            }
            line
        })
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .map(|mut line| {
            let mut str = String::new();
            str.push(line.chars().next().unwrap());
            str.push(line.pop().unwrap());
            str
        })
        .map(|line| line.parse::<u32>())
        .filter_map(|num| num.ok())
        .sum();
    println!("{}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
