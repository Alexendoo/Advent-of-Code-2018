extern crate ndarray;
extern crate itertools;

use itertools::Itertools;
use ndarray::Array2;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
    idx: usize,
}

#[derive(Debug)]
struct Bounds {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn get_range(coords: &[Coordinate], f: impl Fn(&&Coordinate) -> usize) -> (usize, usize) {
    match coords.into_iter().minmax_by_key(&f) {
        itertools::MinMaxResult::MinMax(min, max) => (f(&min), f(&max)),
        _ => panic!(),
    }
}

impl Bounds {
    fn new(coords: &[Coordinate]) -> Self {
        let (x1, x2) = get_range(coords, |coord| coord.x);
        let (y1, y2) = get_range(coords, |coord| coord.y);

        Bounds {
            x: x1,
            y: y1,
            width: 1 + x2 - x1,
            height: 1 + y2 - y1,
        }
    }
}


#[derive(Debug)]
struct Distance<'a> {
    from: &'a Coordinate,
    size: usize,
}

fn difference(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

impl<'a> Distance<'a> {
    fn new(point: (usize, usize), coord: &'a Coordinate) -> Self {
        Distance {
            from: coord,
            size: difference(point.0, coord.x) + difference(point.1, coord.y),
        }
    }
}

fn main() {
    let input = include_str!("./input");

    let coordinates: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut digits = line.split(", ");

            let mut parse = || digits.next().unwrap().parse().unwrap();

            Coordinate {
                x: parse(),
                y: parse(),
                idx: i,
            }
        })
        .collect();


    let bounds = Bounds::new(&coordinates);

    let grid = Array2::from_shape_fn((bounds.width, bounds.height), |(x, y)| {
        let point = (x + bounds.x, y + bounds.y);

        let (distance, uniq) = coordinates
            .iter()
            .map(|coord| Distance::new(point, coord))
            .fold(
                (
                    Distance {
                        from: &coordinates[0],
                        size: usize::max_value(),
                    },
                    true,
                ),
                |(old, uniq), new| match new.size.cmp(&old.size) {
                    Ordering::Equal => (old, false),
                    Ordering::Greater => (old, uniq),
                    Ordering::Less => (new, true),
                },
            );

        if uniq {
            Some(distance.from.idx)
        } else {
            None
        }
    });

    let exteriors = [
        grid.row(0),
        grid.row(grid.rows() - 1),
        grid.column(0),
        grid.column(grid.cols() - 1),
    ];

    let infinities: HashSet<_> = exteriors
        .into_iter()
        .flat_map(|x| x)
        .filter_map(|&i| i)
        .collect();

    let counts = grid
        .iter()
        .filter_map(|opt| opt.filter(|idx| !infinities.contains(idx)))
        .fold(HashMap::new(), |mut map, idx| {
            *map.entry(idx).or_insert(0) += 1;

            map
        });

    let largest = counts.values().max().unwrap();

    println!("Part 1: {}", largest);

    let grid2 = Array2::from_shape_fn(grid.dim(), |(x, y)| {
        let point = (x + bounds.x, y + bounds.y);


    });
}
