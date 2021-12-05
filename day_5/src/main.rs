use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

type HydrothermalUnit = (i32, i32, i32, i32);

fn main() {
    let inputs = include_str!("../input.txt");

    let inputs = inputs
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| s.split(","))
                .flatten()
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<HydrothermalUnit>>();

    let horizontal_or_vertical = inputs
        .iter()
        .copied()
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2);

    let all = inputs.iter().copied();

    println!(
        "horizontal_or_vertical {:?}",
        count_overlaping_points(horizontal_or_vertical)
    );

    println!("all {:?}", count_overlaping_points(all));
}

// all credits for this function goes to https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/05.rs
fn count_overlaping_points(points: impl Iterator<Item = HydrothermalUnit>) -> u32 {
    let mut overlaping_points: HashMap<(i32, i32), i32> = HashMap::new();

    for (x1, y1, x2, y2) in points {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let (mut x, mut y) = (x1, y1);

        while (x, y) != (x2 + dx, y2 + dy) {
            let key = (x, y);
            *overlaping_points.entry(key).or_insert(0) += 1;

            x += dx;
            y += dy;
        }
    }
    overlaping_points.values().filter(|&v| *v > 1).count() as u32
}
