use std::{fs, collections::HashSet};

fn create_set(line: &str) -> HashSet<u64>{
    line.split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

pub fn part1() {
    let contents = fs::read_to_string("input/day4.txt").unwrap();
    let mut total_points: u64 = 0;
    for line in contents.lines() {
        let l: Vec<_> = line.split(':').collect();
        let results: Vec<_> = l[1].split('|').collect();
        let winners = create_set(results[0]);
        let guesses = create_set(results[1]);
        let matches = guesses.intersection(&winners).count();
        if matches > 0 {
            total_points += 1 << (matches-1);
        }
    }

    println!("{total_points}");
}


pub fn part2() {
    let contents = fs::read_to_string("input/day4.txt").unwrap();
    
    let mut games: Vec<i32> = (1..=190).map(|_| 1).collect();
    for (i, line) in contents.lines().enumerate() {
        let l: Vec<_> = line.split(':').collect();
        let results: Vec<_> = l[1].split('|').collect();
        let winners = create_set(results[0]);
        let guesses = create_set(results[1]);
        let matches = guesses.intersection(&winners).count();

        let card_count = games[i];
        for j in i+1..i+1+matches {
            games[j] += card_count;
        }
    }

    let total_cards: i32 = games.iter().sum();
    println!("{total_cards}");
}