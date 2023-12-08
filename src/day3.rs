use std::{fs, collections::{HashMap, HashSet}};


fn is_symbol_check(c: char) -> bool {
    return !c.is_ascii_digit() && c != '.';
}


fn symbol_found_around (x: usize, y: usize, rows: usize, cols: usize, grid: &Vec<Vec<char>> ) -> bool {
    // Check left
    let mut is_symbol = false;
    if x > 0 {
        is_symbol |= is_symbol_check(grid[y][x-1]);
    }

    // check right
    if x < cols-1 {
        is_symbol |= is_symbol_check(grid[y][x+1]);
    }

    // check above
    if y > 0 {
        is_symbol |= is_symbol_check(grid[y-1][x]);
    }

    // check below
    if y < rows-1 {
        is_symbol |= is_symbol_check(grid[y+1][x]);
    }

    // top left
    if x > 0 && y > 0 {
        is_symbol |= is_symbol_check(grid[y-1][x-1]);
    }

    //top right
    if x < cols-1 && y > 0 {
        is_symbol |= is_symbol_check(grid[y-1][x+1]);
    }

    // bottom left
    if x > 0 && y < rows-1 {
        is_symbol |= is_symbol_check(grid[y+1][x-1]);
    }

    // bottom right
    if x < cols-1 && y < rows-1 {
        is_symbol |= is_symbol_check(grid[y+1][x+1]);
    }

    return is_symbol;

}


pub fn part1() {
    let contents = fs::read_to_string("input/puzzle3.txt").unwrap();

    // Create grid from list
    let mut grid = Vec::new();
    for line in contents.lines() {
        let v: Vec<char> = line.chars().collect();
        grid.push(v);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        
        let mut symbol_seen = false;
        let mut val: u64 = 0;
        for (j, c) in row.iter().enumerate() {
            
            if c.is_ascii_digit() {
                let digit = (*c as u8) - ('0' as u8); 
                val *= 10;
                val += digit as u64;
                symbol_seen |= symbol_found_around(j, i, rows, cols, &grid);

            } else {
                if symbol_seen {
                    //println!("Number found: {val}");
                    sum += val;
                }
                symbol_seen = false;
                val = 0;
            }
        }
        if symbol_seen {
            sum += val;
        }
        
    }

    println!("\n{sum}");
        
}

fn is_star_check(c: char) -> bool {
    c == '*'
}


fn stars_found_around (x: usize, y: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut stars = Vec::new();

    // Create a helper function
    let add_star_if_found = |stars: &mut Vec<(usize, usize)>, grid: &Vec<Vec<char>>, x: usize, y: usize| {
        if is_star_check(grid[y][x]) {
            stars.push((x, y));
        }
    };

    // Check left
    if x > 0 {
        add_star_if_found(&mut stars, grid, x-1, y);
    }

    // check right
    if x < cols-1 {
        add_star_if_found(&mut stars, grid, x+1, y);
    }

    // check above
    if y > 0 {
        add_star_if_found(&mut stars, grid, x, y-1);
    }

    // check below
    if y < rows-1 {
        add_star_if_found(&mut stars, grid, x, y+1);
    }

    // top left
    if x > 0 && y > 0 {
        add_star_if_found(&mut stars, grid, x-1, y-1);
    }

    //top right
    if x < cols-1 && y > 0 {
        add_star_if_found(&mut stars, grid, x+1, y-1);
    }

    // bottom left
    if x > 0 && y < rows-1 {
        add_star_if_found(&mut stars, grid, x-1, y+1);
    }

    // bottom right
    if x < cols-1 && y < rows-1 {
        add_star_if_found(&mut stars, grid, x+1, y+1);
    }

    return stars;

}


pub fn part2() {
    let contents = fs::read_to_string("input/puzzle3.txt").unwrap();

    // Create grid from list
    let mut grid = Vec::new();
    for line in contents.lines() {
        let v: Vec<char> = line.chars().collect();
        grid.push(v);
    }

    let cols = grid[0].len();

    let mut stars = HashMap::<(usize, usize), Vec<u64>>::new();
    for (i, row) in grid.iter().enumerate() {
        
        let mut val: u64 = 0;
        let mut num_found = false;
        let mut stars_current_number =  HashSet::<(usize, usize)>::new();
        for (j, c) in row.iter().enumerate() {
            
            if c.is_ascii_digit() {
                num_found = true;
                let digit = (*c as u8) - ('0' as u8); 
                val *= 10;
                val += digit as u64;
                
                // Add in all the stars around this number
                let found_stars = stars_found_around(j, i, &grid);

                for star_coords in found_stars {
                    stars_current_number.insert(star_coords);
                }

            }
            
            if !c.is_ascii_digit() || j == (cols-1) {
                if num_found {
                    num_found = false;
              
                    for star_coord in stars_current_number.iter() {
                        stars.entry(star_coord.clone())
                            .and_modify(|v| {v.push(val)})
                            .or_insert(vec![val]);
                    }
                    stars_current_number.clear();
                }

                val = 0;
            }
        }

    }

    for ((x,y), nums) in &stars {
        print!("{x},{y}: ");
        for num in nums {
            print!("{num}, ");
        }
        println!("");
        
    }

    // Calculate sum
    let sum: u64 = stars.into_values()
        .filter_map(|v| (v.len() == 2).then(|| v[0]*v[1]))
        .sum();

    println!("{sum}");
        
}