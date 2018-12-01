use std::collections::HashSet;

fn main() {
    let input = include_str!("./input");

    let changes = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap());

    println!("Part 1: {}", changes.clone().sum::<i64>());

    let mut seen: HashSet<i64> = HashSet::new();

    let repeated = changes
        .cycle()
        .scan(0, |sum, x| {
            *sum += x;

            Some(*sum)
        })
        .find(|&x| !seen.insert(x))
        .unwrap();

    println!("Part 2: {}", repeated)
}
