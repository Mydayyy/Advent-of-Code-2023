use std::collections::HashMap;

use aoc23::get_input;

#[derive(PartialEq)]
enum States {
    None,
    Number,
    NewLine,
}

fn main() {
    let input = get_input(3);
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    let mut dx: i32 = 0;
    let mut dy: i32 = 0;

    for (idx_line, line) in input.lines().enumerate() {
        for (idx_char, char) in line.chars().enumerate() {
            map.insert((idx_char as i32, idx_line as i32), char);
            dx = dx.max(idx_char as i32);
        }
        dy = dy.max(idx_line as i32);
    }

    let is_valid_index = |x: i32, y: i32| {
        return x >= 0 && y >= 0 && x <= dx && y <= dy;
    };

    let has_adjacent_symbol = |x: i32, y: i32| {
        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (-1, 1), (1, -1)] {
            let pos = (x + offset.0, y + offset.1);
            if !is_valid_index(pos.0, pos.1) { continue; };
            let c = map[&pos];
            if !c.is_numeric() && c != '.' {
                return true;
            }
        }
        return false;
    };

    let get_adjacent_gears = |x: i32, y: i32| {
        let mut gears: Vec<(i32, i32)> = vec![];
        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (-1, 1), (1, -1)] {
            let pos = (x + offset.0, y + offset.1);
            if !is_valid_index(pos.0, pos.1) { continue; };
            let c = map[&pos];
            if c == '*' {
                gears.push(pos)
            }
        }
        return gears;
    };

    let mut state = States::None;
    let mut current_number: i32 = 0;
    let mut number_had_symbol = false;
    let mut sum: i32 = 0;
    let mut number_adjacent_gears: HashMap<(i32, i32), u8> = HashMap::new();
    for y in 0..=dy {
        for x in 0..=dx {
            let c = map[&(x, y)];
            if (!c.is_numeric() && state == States::Number) || (state == States::NewLine && current_number > 0) {
                sum += if number_had_symbol { current_number } else { 0 };

                for (gear, _) in number_adjacent_gears.clone() {
                    gears.entry(gear).and_modify(|x| x.push(current_number)).or_insert_with(|| vec![current_number]);
                }

                current_number = 0;
                state = States::None;
                number_had_symbol = false;
                number_adjacent_gears.clear();
            }
            if c.is_numeric() {
                state = States::Number;
                current_number = current_number * 10 + c.to_digit(10).unwrap() as i32;
                number_had_symbol = number_had_symbol || has_adjacent_symbol(x, y);

                let gears = get_adjacent_gears(x, y);
                for gear in gears {
                    number_adjacent_gears.insert(gear, 0);
                }
            }
        }
    }

    println!("{:?}", sum);
    println!("{:?}", gears.iter().filter(|(_, g)| g.len() == 2).collect::<HashMap<_, _>>().values().map(|x| x.iter().product::<i32>()).sum::<i32>())
}