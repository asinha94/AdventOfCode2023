use std::fs;
use std::collections::HashMap;
use num::integer::lcm;

struct Node<'a> {
    left: &'a str,
    right: &'a str
}


pub fn part1() {
    let content = fs::read_to_string("input/day8.txt").unwrap();
    let lines: Vec<_> = content.lines().collect();

    let directions = lines[0];

    let mut graph= HashMap::new();
    
    for i in 2..lines.len() {
        let line = lines[i];
        let root = line.get(0..3).unwrap();
        let left = line.get(7..10).unwrap();
        let right = line.get(12..15).unwrap();

        let n = Node {left, right};
        graph.insert(root, n);
    }

    let mut count = 0;
    let mut start = "AAA";
    loop {
        for c in directions.chars() {

            if start == "ZZZ" {
                println!("{count}");
                return;
            }

            start = match c {
                'L' => graph[start].left,
                'R' => graph[start].right,
                _ => panic!("Didn't get L/R instruction!")
            };
            count += 1;

        }

    };

}

pub fn part2() {
    let content = fs::read_to_string("input/day8.txt").unwrap();
    let lines: Vec<_> = content.lines().collect();

    let directions = lines[0];

    let mut graph= HashMap::new();
    
    for i in 2..lines.len() {
        let line = lines[i];
        let root = line.get(0..3).unwrap();
        let left = line.get(7..10).unwrap();
        let right = line.get(12..15).unwrap();

        let n = Node {left, right};
        graph.insert(root, n);
    }

    
    let mut start: Vec<_> = graph.keys().filter(|k| k.ends_with('A')).collect();
    let mut lcms = Vec::new();

    for start in start {
        let mut count: usize = 0;
        let mut skey = start;
        'inf: loop {
            for c in directions.chars() {            
                if skey.ends_with('Z') {
                    break 'inf count;
                }
    
                skey = match graph.get(skey) {
                    None => panic!("Somehow no key {skey}"),
                    Some(n) => match c {
                        'L' => &n.left,
                        'R' => &n.right,
                        _ => panic!("Didn't get L/R instruction!")
                    }
                };
                count += 1;
    
            }
        };
        lcms.push(count);
    }

    let mut fin_lcm = 1;
    for l in lcms {
        fin_lcm = lcm(fin_lcm, l);
    }
    println!("{fin_lcm}");

}
