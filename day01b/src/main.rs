use std::fs;

fn main () {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    println!("{}", count_callories(&input));
}

fn count_callories(input: &String) -> i32 {
    let mut temp_count: i32 = 0;
    let mut all_callories: Vec<i32> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            all_callories.push(temp_count);
            temp_count = 0;
        } else {
            let line_value: i32 = line.parse().unwrap();
            temp_count += line_value;
        }
    }

    all_callories.sort_by(|a, b| b.cmp(a));
    all_callories[0] + all_callories[1] + all_callories[2]
}
