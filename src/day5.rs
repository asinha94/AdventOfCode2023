use std::cmp::min;
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

    fn disp(&self) -> String {
        format!("{}->{}", self.source, self.source+self.range-1)
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
    let content = fs::read_to_string("input/day5.txt").unwrap();
    
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

impl SeedRange {
    fn clone(&self) -> SeedRange {
        SeedRange {
            start: self.start,
            range: self.range
        }
    }
}


fn transform2(map: &Vec<RangeTransform>, input: &SeedRange) -> Vec<SeedRange> {

    let r = map.binary_search_by(|k| k.cmp(input.start));
    let idx = if r.is_err() { r.unwrap_err() } else { r.unwrap() };

    let mut transformed = vec![];
    let mut remaining = input.clone();
    for m in &map[idx..] {

        // remove portion of range that is passed through as-is
        if remaining.start < m.source {
            let diff_range = m.source - remaining.start;
            if diff_range >= remaining.range {
                transformed.push(remaining);
                return transformed;
            }

            remaining.start += diff_range;
            remaining.range -= diff_range;
        }

        // At this point the remaining range has to be at least equal
        // to the transform start, so find the base
        let base = remaining.start - m.source;
        let transformed_range = min(remaining.range, m.range - base);
        transformed.push(SeedRange {
            start: m.desitnation + base,
            range: transformed_range
        });

        remaining.start += transformed_range;
        remaining.range -= transformed_range;

        if remaining.range == 0 {
            return transformed;
        }
    }

    if remaining.range != 0 {
        transformed.push(remaining);
    }
    
    transformed
}


pub fn part2() {
    let content = fs::read_to_string("input/day5.txt").unwrap();
    
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

    let mut inputs: Vec<_> = seeds.iter()
        .map(|x| x.clone())
        .collect();

    for map in maps {
        // Transform all the inputs and place into this vector
        
        let mut future_inputs = vec![];
        for input in inputs {
            let mut transformed_inputs = transform2(&map, &input);
            future_inputs.append(&mut transformed_inputs);            
        }

        inputs = future_inputs;

        // For a more functional approach
        /* inputs = inputs.iter()
                       .flat_map(|x| transform2(&map, x))
                       .collect(); */
    }

    inputs.sort_by_key(|k| k.start);
    let lowest = &inputs[0];
    println!("{}", lowest.start);
    
}