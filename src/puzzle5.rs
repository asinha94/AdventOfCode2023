use std::fs;
use std::cmp;

struct RangeTransform {
    source: u64,
    desitnation: u64,
    range: u64
}

impl RangeTransform {

    fn transform(&self, input: u64) -> u64 {
        self.desitnation + (input - self.source)
    }

    fn cmp(&self, v: u64) -> cmp::Ordering {

        let upper_bound = self.source + self.range - 1;
        if upper_bound < v  {
            return cmp::Ordering::Less;
        }

        if self.source > v {
            return cmp::Ordering::Greater;
        }



        return cmp::Ordering::Equal;
    }
}


fn transform(map: &Vec<RangeTransform>, input: u64) -> u64 {

    let r = map.binary_search_by(|k| k.cmp(input));
    if r.is_err() {
        return input;
    }

    let i = r.unwrap();
    let t = &map[i];
    t.transform(input)
}



pub fn part1() {
    let content = fs::read_to_string("input/puzzle5.txt").unwrap();
    
    let lines: Vec<_> = content.lines().collect();
    let seed_values: Vec<_> = lines[0].split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut maps = vec![];
    let mut ranges = vec![];
    let mut skip_again = true;
    for line in &lines[2..] {

        if line.len() == 0 {
            // Insert the map into the vector
            let mut r = ranges;
            r.sort_by_key(|x: &RangeTransform| x.source);
            
            maps.push(r);
            ranges = vec![];
            skip_again = true;
            continue;
        }

        if skip_again {
            skip_again = false; 
            continue;
        }

        let vals: Vec<_> = line.split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        let desitnation = vals[0];
        let source = vals[1];
        let range = vals[2];

        ranges.push(RangeTransform {
            source: source,
            desitnation: desitnation,
            range: range
        });
    }

    let mut locations = vec![];
    for seed in seed_values {
        /* 
        // Same thing as the functional form below
        let mut s = seed; 
        for map in &maps {
            s = transform(&map, s);
        } */

        let s = maps.iter().fold(seed, |acc, map| transform(map, acc));
        locations.push(s);
    }
    
    locations.sort();
    println!("{}",locations[0]);
    
    /* 
    let sv = 2880930400;
    for map in &maps[0] {
        let between = map.source < sv && sv < map.source+map.range;
        let gt = map.cmp(sv).is_gt();
        let lt = map.cmp(sv).is_lt();
        let ge = map.cmp(sv).is_ge();
        let le = map.cmp(sv).is_le();
        println!("{sv} between {}..{}? {between}. lt {lt}, gt {gt}, ge {ge}, le {le}", map.source, map.source+map.range);
    }*/

}