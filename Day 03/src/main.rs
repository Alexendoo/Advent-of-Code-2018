#![feature(nll)]

extern crate regex;

use regex::{Regex, Matches};
use std::collections::BTreeMap;

struct Claim {
    idx: usize,
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
            idx: next(),
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

    let fabric = input.lines()
        .map(|line| re.find_iter(line))
        .map(Claim::new)
        .flat_map(|Claim { x, y, width, height, .. }| {
            (x..x + width)
                .flat_map(move |x| (y..y + height).map(move |y| (x, y)))
        })
        .fold(BTreeMap::<_, BTreeMap<_, _>>::new(), |mut fabric, (x, y)| {
            let row = fabric.entry(x).or_default();
            let entry = row.entry(y).or_insert(0);

            *entry += 1;

            fabric
        });

    let count = fabric.values()
        .flat_map(|row| row.values())
        .cloned()
        .filter(|&u| u > 1)
        .count();

    println!("Part 1: {}", count);
}