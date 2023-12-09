use std::fs;

fn calculate_winning_possibilities(time: u64, distance: u64) -> u64 {
    let mut count = 0;

    for t in 1..(time-1) {
        let d = (time-t) * t;
        if d > distance {
            count += 1;
        }
    }

    count
}


pub fn part1() {
    let content = fs::read_to_string("input/day6.txt").unwrap();
    let lines: Vec<_> = content.lines().collect();
    
    let times: Vec<_> = lines[0].split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let distances: Vec<_> = lines[1].split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let rounds: Vec<(u64, u64)> = times.iter()
                                       .zip(distances)
                                       .map(|u| (u.0.clone(),u.1))
                                       .collect();

    let mut val = 1;
    for round in rounds {
        val *= calculate_winning_possibilities(round.0, round.1);
    }

    println!("{val}");
}


pub fn part2() {
    let content = fs::read_to_string("input/day6.txt").unwrap();
    let lines: Vec<_> = content.lines().collect();

    let f = |x: u64| {
        let xf64 = x as f64;
        let pow = 1 + xf64.log10() as u64;
        10_u64.pow(pow as u32)
    };

    let time = lines[0].split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .fold(0, |acc, e | acc * f(e.clone()) + e );

    let distance = lines[1].split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .fold(0, |acc, e | acc * f(e.clone()) + e );

    let ways = calculate_winning_possibilities(time, distance);
    println!("{ways}");
}