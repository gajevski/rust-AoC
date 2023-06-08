use std::fs;

fn main () {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    count_score(&input);
}

fn count_score(input: &String) {
    let contents: u32 = input.lines().map(|s| {
        match s {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        }
    }).into_iter().sum();
    println!("{:?}", contents);
}