use std::error::Error;
use std::io::{self, BufRead};

fn fully_contains(r1: (u32, u32), r2: (u32, u32)) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.1 || r2.0 <= r1.0 && r2.1 >= r1.1
}

fn has_overlap(r1: (u32, u32), r2: (u32, u32)) -> bool {
    r1.0 >= r2.0 && r1.0 <= r2.1 || r2.0 >= r1.0 && r2.0 <= r1.1
}

fn part1(input: &Vec<Vec<Vec<u32>>>) -> u32 {
    input
        .iter()
        .map(|v| {
            let r1 = (v[0][0], v[0][1]);
            let r2 = (v[1][0], v[1][1]);

            if fully_contains(r1, r2) {
                1
            } else {
                0
            }
        })
        .sum::<_>()
}

fn part2(input: &Vec<Vec<Vec<u32>>>) -> u32 {
    input
        .iter()
        .map(|v| {
            let r1 = (v[0][0], v[0][1]);
            let r2 = (v[1][0], v[1][1]);

            if has_overlap(r1, r2) {
                1
            } else {
                0
            }
        })
        .sum::<_>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let lines = input
        .iter()
        .map(|s| {
            s.split(",")
                .map(|d| {
                    d.split("-")
                        .map(|e| e.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}, {}", part1(&lines), part2(&lines));

    Ok(())
}