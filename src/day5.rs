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

    fn transform2(&self, s: &SeedRange) {

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
        let s = maps.iter()
            .fold(seed, |acc, map| transform(map, acc));
        locations.push(s);
    }
    
    locations.sort();
    println!("{}",locations[0]);
    
}

struct SeedRange {
    start: u64,
    range: u64
}


fn transform2(map: &Vec<RangeTransform>, input: SeedRange) -> SeedRange {

    let r = map.binary_search_by(|k| k.cmp(input));
    if r.is_err() {
        return input;
    }

    let i = r.unwrap();
    let t = &map[i];
    t.transform(input);

    SeedRange{start: 0, range: 0}
}


pub fn part2() {
    let content = fs::read_to_string("input/puzzle5.txt").unwrap();
    
    let lines: Vec<_> = content.lines().collect();
    let seed_values: Vec<_> = lines[0].split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let pairs = seed_values.len() / 2;
    let seeds: Vec<_> = (0..pairs).map(|i| SeedRange{start: seed_values[2*i], range: seed_values[2*i+1]})
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
    for seed in seeds {
        let s = maps.iter()
            .fold(seed, |acc, map| transform2(map, &acc));
        locations.push(s);
    }
    
    locations.sort();
    println!("{}",locations[0]);
    
}