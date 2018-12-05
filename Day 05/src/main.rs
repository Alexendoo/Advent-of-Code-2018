extern crate rayon;

use rayon::prelude::*;

fn opposite_case_equal(a: u8, b: u8) -> bool {
    (a.is_ascii_lowercase() != b.is_ascii_lowercase())
        && a.eq_ignore_ascii_case(&b)
}

fn solve(iter: impl Iterator<Item = u8>) -> usize {
    let mut buffer = Vec::new();

    for a in iter {
        if let Some(&b) = buffer.last() {
            if !opposite_case_equal(a, b) {
                buffer.push(a);
            } else {
                buffer.pop();
            }
        } else {
            buffer.push(a);
        }
    }

    buffer.len()
}

fn main() {
    let input = include_str!("./input").trim();

    let len = solve(input.bytes());

    println!("Part 1: {}", len);

    let min = (b'a'..(b'z' + 1))
        .into_par_iter()
        .map(|letter| {
            let iter = input.bytes()
                .filter(|byte| !byte.eq_ignore_ascii_case(&letter));

            solve(iter)
        })
        .min()
        .unwrap();

    println!("Part 2: {}", min);
}
