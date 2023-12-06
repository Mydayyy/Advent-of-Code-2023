use std::collections::HashMap;

use aoc23::get_input;

fn main() {
    let input = get_input(4);

    let mut copies: HashMap<i32, i32> = HashMap::new();
    let mut sum1 = 0;
    let mut processed = 0;
    for (i, line) in input.lines().enumerate() {
        processed += 1;
        let winning: Vec<_> = line.split_once(": ").unwrap().1.split_once(" | ").unwrap().0.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()).collect();
        let mine: Vec<_> = line.split_once(": ").unwrap().1.split_once(" | ").unwrap().1.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()).collect();

        let winners: Vec<_> = mine.iter().filter(|x| winning.contains(x)).collect();

        let duplicates = copies.get(&(i as i32)).unwrap_or(&0).clone();
        processed += duplicates;

        if winners.len() == 0 { continue; }

        for (offset, _) in winners.iter().enumerate() {
            copies.entry((i + offset + 1) as i32).and_modify(|x| *x += duplicates + 1).or_insert(duplicates + 1);
        }

        sum1 += 2_i32.pow((winners.len() - 1) as u32);
    }
    println!("{}", sum1);
    println!("{}", processed);
}