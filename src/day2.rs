use std::fs;
use std::fmt;

struct RoundGuesses {
    red: i32,
    green: i32,
    blue: i32,
}

impl RoundGuesses {
    fn new(r: i32, g: i32, b: i32) -> RoundGuesses {
        RoundGuesses {
            red: r,
            green: g,
            blue: b
        }
    }

    fn add_guess(&mut self, color: &str, guess: i32) {
        match color.trim().to_lowercase().as_str() {
            "red" => self.red += guess,
            "green" => self.green += guess,
            "blue" => self.blue += guess,
            _ => println!("Unknown color found: {color}")
        }
    }

    fn assign_max_mut(&mut self, other: &RoundGuesses) {
        self.red = std::cmp::max(self.red, other.red);
        self.green = std::cmp::max(self.green, other.green);
        self.blue = std::cmp::max(self.blue, other.blue);
    }

    fn can_be_played(&self, constraints: &RoundGuesses) -> bool {
        return self.red <= constraints.red
            && self.green <= constraints.green
            && self.blue <= constraints.blue
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl fmt::Display for RoundGuesses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Red: {}, Green: {}, Blue: {}", self.red, self.green, self.blue)
    }
}

pub fn part1and2() {
    let contents = fs::read_to_string("input/puzzle2.txt").unwrap();

    let mut games = Vec::new();

    for line in contents.lines() {
        let i = 1 + line.find(':').unwrap();

        let mut game_total = RoundGuesses::new(0, 0, 0);
        // For each round of the game
        for round in line[i..].split(';') {

            let mut r = RoundGuesses::new(0, 0, 0);
            // For each color in the round
            for color_match in round.split(',') {
                let (num, color) = color_match.trim().split_once(' ').unwrap();

                let val: i32 = num.parse().unwrap();
                r.add_guess(color, val);
            }

            // Once round has been parsed, append into game number vector
            game_total.assign_max_mut(&r);
        }

        // Add accumulated round maximums
        games.push(game_total);

    }

    let mut sum_of_game_ids: usize = 0;
    let mut sum_of_powers: i32 = 0;
    let constraints = RoundGuesses::new(12, 13, 14);
    for (i, game) in games.iter().enumerate() {
        sum_of_powers += game.power();
        if game.can_be_played(&constraints) {
            sum_of_game_ids += i + 1;
        }
    }

    println!("Sum of Game IDs: {sum_of_game_ids}");
    println!("Sum of powers: {sum_of_powers}");

}
