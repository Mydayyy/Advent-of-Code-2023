use std::collections::HashMap;

use aoc23::get_input;

fn main() {
    let input = get_input(5);

    let mut seeds1: Vec<i64> = vec![];
    let mut maps: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();

    let mut current_map = "".to_string();
    for line in input.lines() {
        if line.starts_with("seeds:") {
            seeds1.extend(line.replace("seeds: ", "").split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>());
        }

        if line.is_empty() {}

        if line.ends_with(" map:") {
            current_map = line.replace(" map:", "").clone();
            maps.entry(current_map.clone()).or_insert(Default::default());
        }

        if line.chars().next().unwrap_or(' ').is_numeric() {
            let mut n = line.split(" ").map(|x| x.parse::<i64>().unwrap());
            let (destination, source, range) = (n.next().unwrap(), n.next().unwrap(), n.next().unwrap());
            maps.get_mut(&current_map).unwrap().push((source, destination, range));
        }
    }

    let translate_map_entry = |map_name: &str, number: i64, reverse: bool| {
        let ranges = maps.get(map_name).unwrap();
        for range in ranges {
            let source = if reverse { range.1 } else { range.0 };
            let destination = if reverse { range.0 } else { range.1 };
            let r = range.2;
            if number >= source && number < source + r {
                return destination + (number - source);
            }
        }
        return number;
    };

    let mut lowest1: i64 = i64::MAX;
    for (_idx, seed) in seeds1.iter().enumerate() {
        let soil = translate_map_entry("seed-to-soil", *seed, false);
        let fertilizer = translate_map_entry("soil-to-fertilizer", soil, false);
        let water = translate_map_entry("fertilizer-to-water", fertilizer, false);
        let light = translate_map_entry("water-to-light", water, false);
        let temperature = translate_map_entry("light-to-temperature", light, false);
        let humidity = translate_map_entry("temperature-to-humidity", temperature, false);
        let location = translate_map_entry("humidity-to-location", humidity, false);
        lowest1 = lowest1.min(location);
    }

    let mut lowest2: i64 = i64::MAX;
    'outer: for location in 0i64.. {
        if location % 100000 == 0 {
            println!("location {}", location);
        }

        let humidity = translate_map_entry("humidity-to-location", location, true);
        let temperature = translate_map_entry("temperature-to-humidity", humidity, true);
        let light = translate_map_entry("light-to-temperature", temperature, true);
        let water = translate_map_entry("water-to-light", light, true);
        let fertilizer = translate_map_entry("fertilizer-to-water", water, true);
        let soil = translate_map_entry("soil-to-fertilizer", fertilizer, true);
        let s = translate_map_entry("seed-to-soil", soil, true);

        for (idx, seed) in seeds1.iter().enumerate().step_by(2) {
            let to = seeds1[idx + 1];
            if s >= *seed && s < *seed + to {
                lowest2 = location;
                break 'outer;
            }
        }
    }


    println!("{}", lowest1);
    println!("{}", lowest2);
}