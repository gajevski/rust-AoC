use std::fs;

fn main () {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    count_score(&input);
}

fn count_score(input: &String) {
    let contents: u32 = input.lines().map(|s| {
        match s {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        }
    }).into_iter().sum();
    println!("{:?}", contents);
}