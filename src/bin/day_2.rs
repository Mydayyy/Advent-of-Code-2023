use aoc23::get_input;

fn main() {
    let input = get_input(2);

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input.lines() {
        let (id, data) = line.split_once(": ").unwrap();
        let game_id = id.replace("Game ", "").parse::<u32>().unwrap();

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;


        let games = data.split("; ");
        for game in games {
            let cubes = game.split(", ");
            for cube in cubes {
                let (cube_count_str, cube_color) = cube.split_once(" ").unwrap();
                let cube_count = cube_count_str.parse::<u32>().unwrap();

                if cube_color == "red" {
                    red = red.max(cube_count);
                }
                if cube_color == "blue" {
                    blue = blue.max(cube_count);
                }
                if cube_color == "green" {
                    green = green.max(cube_count);
                }
            }
        }

        let is_possible = !(red > 12 || green > 13 || blue > 14);
        if is_possible {
            sum1 += game_id;
        }
        sum2 += green * red * blue;
    }

    println!("{}", sum1);
    println!("{}", sum2);
}