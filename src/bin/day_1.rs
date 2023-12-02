use aoc23::get_input;

fn main() {
    let input = get_input(1);

    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;
    for mut line in input.lines() {
        let digits: Vec<u8> = line.chars().filter_map(|x| x.to_digit(10).map(|x| x as u8)).collect();
        let number = digits[digits.len() - 1] + digits[0] * 10;
        sum1 += number as u64;

        while !line.chars().next().unwrap().is_numeric() && !numbers.iter().any(|num| line.starts_with(num)) {
            line = &line[1..];
        }

        while !line.chars().last().unwrap().is_numeric() && !numbers.iter().any(|num| line.ends_with(num)) {
            line = &line[..line.len() - 1];
        }

        let mut first: u64 = 0;
        let mut last: u64 = 0;
        for (idx, num) in numbers.iter().enumerate() {
            if line.starts_with(num) {
                first = idx as u64 + 1;
            }
            if line.ends_with(num) {
                last = idx as u64 + 1;
            }
        }

        if line.chars().next().unwrap().is_numeric() {
            first = line.chars().next().unwrap().to_digit(10).unwrap() as u64;
        }
        if line.chars().last().unwrap().is_numeric() {
            last = line.chars().last().unwrap().to_digit(10).unwrap() as u64;
        }

        sum2 += last + first * 10;
    }
    println!("{}", sum1);
    println!("{}", sum2)
}