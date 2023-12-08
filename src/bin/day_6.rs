use aoc23::get_input;

fn main() {
    let input = get_input(6);
    let lines = input.split('\n').map(|x| x.split_once(":").unwrap().1).collect::<Vec<_>>();
    // let time = input.split('\n').next().unwrap().replace("Time:", "").clone().split(" ");
    // let time = lines[0].split_whitespace();
    let time: Vec<_> = lines[0].split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
    let distance: Vec<_> = lines[1].split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    let part1 = time.iter().zip(distance.iter()).map(|(&t, &d)| (0..t).filter(move |x| x * (t - x) > d).count()).product::<usize>();
    let part2 = [time.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("").parse::<i64>().unwrap()].iter().zip([distance.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("").parse::<i64>().unwrap()].iter()).map(|(&t, &d)| (0..t).filter(move |x| x * (t - x) > d).count()).product::<usize>();
    println!("{:?}", part1);
    println!("{:?}", part2);
}