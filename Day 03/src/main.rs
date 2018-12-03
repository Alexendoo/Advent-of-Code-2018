extern crate regex;

use regex::{Regex, Matches};
use std::collections::{HashMap, HashSet};

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(mut iter: Matches) -> Self {
        let mut next = || {
            iter.next().unwrap().as_str().parse().unwrap()
        };

        Claim {
            id: next(),
            x: next(),
            y: next(),
            width: next(),
            height: next(),
        }
    }
}

fn main() {
    let input = include_str!("./input");

    let re = Regex::new(r"\d+").unwrap();

    let claims = input.lines()
        .map(|line| re.find_iter(line))
        .map(Claim::new);

    let mut fabric = HashMap::new();
    let mut uniques = HashSet::new();

    for Claim { id, x, y, width, height } in claims {
        uniques.insert(id);

        for x in x..x+width {
            for y in y..y+height {
                fabric.entry((x, y))
                    .and_modify(|(count, oid)| {
                        *count += 1;

                        uniques.remove(&id);
                        uniques.remove(&oid);
                    })
                    .or_insert((1, id));
            }
        }
    }

    let count = fabric.values()
        .filter(|&&u| u.0 > 1)
        .count();

    println!("Part 1: {}", count);
    println!("Part 2: {}", uniques.into_iter().next().unwrap());
}