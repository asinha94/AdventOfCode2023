use std::fs;

fn get_diffs(seq: &Vec<i64>) -> Vec<i64> {
    let mut before = seq[0];

    let mut diffs = Vec::new(); 
    for num in seq.get(1..).unwrap() {
        let diff = num - before;
        diffs.push(diff);
        before = *num;
    }

    diffs
}

fn get_next_num_in_seq(seqs: &Vec<i64>) -> i64 {
    let diffs = get_diffs(seqs);
    if diffs.iter().all(|f| f == &0) {
        return 0;
    }

    let last = get_next_num_in_seq(&diffs);
    diffs[0] - last
}

pub fn part1() {
    let content = fs::read_to_string("input/day9.txt").unwrap();
    let seqs: Vec<_> = content.lines()
        .map(|l|
            l.split(' ')
            .filter_map(|f| f.parse::<i64>().ok())
            .collect())
        .collect();

    let sum: i64 = seqs.iter()
        .map(|f: &Vec<_>| f[0] - get_next_num_in_seq(f))
        .sum();

    println!("{sum}");
}