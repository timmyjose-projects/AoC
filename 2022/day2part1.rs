use std::error::Error;
use std::io;

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSS_SCORE: i32 = 0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut score = 0;

    for line in io::stdin().lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let (opponent, myself) = (
            line.chars().nth(0).ok_or("no first character")?,
            line.chars().nth(2).ok_or("no second character")?,
        );

        score += match (opponent, myself) {
            ('A', 'X') => DRAW_SCORE + ROCK_SCORE,
            ('A', 'Y') => WIN_SCORE + PAPER_SCORE,
            ('A', 'Z') => LOSS_SCORE + SCISSORS_SCORE,
            ('B', 'X') => LOSS_SCORE + ROCK_SCORE,
            ('B', 'Y') => DRAW_SCORE + PAPER_SCORE,
            ('B', 'Z') => WIN_SCORE + SCISSORS_SCORE,
            ('C', 'X') => WIN_SCORE + ROCK_SCORE,
            ('C', 'Y') => LOSS_SCORE + PAPER_SCORE,
            ('C', 'Z') => DRAW_SCORE + SCISSORS_SCORE,
            _ => continue,
        };
    }

    println!("{}", score);

    Ok(())
}