use std::fs;


enum Directions {
    North,
    South,
    East,
    West
}


enum PipePiece {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Animal,
    Ground
}


fn match_pipe(c: char) -> PipePiece {
    match c {
        '|' => PipePiece::Vertical,
        '-' => PipePiece::Horizontal,
        'L' => PipePiece::UpRight,
        'F' => PipePiece::DownRight,
        'J' => PipePiece::UpLeft,
        '7' => PipePiece::DownLeft,
        'S' => PipePiece::Animal,
        _ => PipePiece::Ground
    }
}


struct Pipe {
    pipe: char,
    distance: i32
}

fn pipe_directions(piece: PipePiece) -> Vec<Directions> {
    match piece {
        PipePiece::Vertical => vec![Directions::North, Directions::South],
        PipePiece::Horizontal => vec![Directions::West,  Directions::East],
        PipePiece::UpRight => vec![Directions::North, Directions::East],
        PipePiece::DownRight => vec![Directions::South, Directions::East],
        PipePiece::UpLeft => vec![Directions::North, Directions::West],
        PipePiece::DownLeft => vec![Directions::South, Directions::West],
        PipePiece::Animal => vec![Directions::North, Directions::South, Directions::East, Directions::West],
        _ => vec![]
    }
}

fn can_reach_pipe(direction: Directions, pipe: &PipePiece) -> bool {
    match direction {
        Directions::North => match pipe {
            PipePiece::Vertical => true,
            PipePiece::DownLeft => true,
            PipePiece::DownRight => true,
            _ => false
        },
        Directions::South => match pipe {
            PipePiece::Vertical => true,
            PipePiece::UpLeft => true,
            PipePiece::UpRight => true,
            _ => false
        },
        Directions::West => match pipe {
            PipePiece::UpRight => true,
            PipePiece::DownRight => true,
            PipePiece::Horizontal => true,
            _ => false
        },
        Directions::East => match pipe {
            PipePiece::UpLeft => true,
            PipePiece::DownLeft => true,
            PipePiece::Horizontal => true,
            _ => false
        }
    }
}

fn dfs_cells(map: &mut Vec<Vec<Pipe>>, cells: &mut Vec<(usize, usize, i32)>, max_col: usize, max_row: usize) {
    let (x, y, distance) = cells.pop().unwrap();

    let c = &mut map[y][x];
    if c.distance > 0 && c.distance < distance {
        return;
    }

    c.distance = distance;

    let directions = pipe_directions(match_pipe(c.pipe));

    for direction in directions {
        match direction {
            Directions::North => {
                if y > 0 && can_reach_pipe(direction, &match_pipe(map[y-1][x].pipe)) {
                    cells.push((x, y-1, distance+1));
                }
            },
            Directions::South => {
                if y < max_row-1 && can_reach_pipe(direction, &match_pipe(map[y+1][x].pipe))  {
                    cells.push((x, y+1, distance+1));
                }
            },
            Directions::West => {
                if x > 0 && can_reach_pipe(direction, &match_pipe(map[y][x-1].pipe)) {
                    cells.push((x-1, y, distance+1));
                }
            },
            Directions::East => {
                if x < max_col-1 && can_reach_pipe(direction, &match_pipe(map[y][x+1].pipe)) {
                    cells.push((x+1, y, distance+1));
                }
            },
        }
    }

}

pub fn part1() {
    let content = fs::read_to_string("input/day10.txt").unwrap();
    let mut map: Vec<Vec<Pipe>> = content.lines()
        .map(|line| line.chars().map(|c| Pipe {pipe: c, distance: -1})
            .collect())
        .collect();

    let max_row = map.len();
    let max_col = map[0].len();
    let mut x = 0;
    let mut y = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.pipe == 'S' {
                x = j;
                y = i;
            }
        }
    }

    let mut cells = vec![(x, y, 0)];
    while cells.len() > 0 {
        dfs_cells(&mut map, &mut cells, max_col, max_row);
    }
    

    let max = map.iter()
        .map(|row| row.iter().max_by_key(|pipe| pipe.distance).unwrap())
        .max_by_key(|pipe| pipe.distance)
        .unwrap();
    println!("{}", max.distance);
    
}