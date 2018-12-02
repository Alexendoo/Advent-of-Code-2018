extern crate onig;

use std::collections::HashMap;
use onig::Regex;

fn main() {
    let input = include_str!("./input");

    let (twos, threes) = input
        .lines()
        .map(|line| line
            .chars()
            .fold(HashMap::new(), |mut map, ch| {
                *map.entry(ch).or_insert(0) += 1;

                map
            }))
        .fold((0, 0), |(twos, threes), counts| (
            if counts.values().any(|&v| v == 2) { twos + 1 } else { twos },
            if counts.values().any(|&v| v == 3) { threes + 1 } else { threes },
        ));

    println!("Part 1: {}", twos * threes);

    let re = Regex::new(r#"(?x)
        ^
        (.*)
        (.)
        (.*)
        \n
        (?:.*\n)*
        \1
        (?!\2)
        .
        \3
        $
    "#).unwrap();

    let captures = re.captures(input).unwrap();

    println!("Part 2: {}{}", captures.at(1).unwrap(), captures.at(3).unwrap());
}
