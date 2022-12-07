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
            ('A', 'X') => SCISSORS_SCORE + LOSS_SCORE,
            ('A', 'Y') => DRAW_SCORE + ROCK_SCORE,
            ('A', 'Z') => PAPER_SCORE + WIN_SCORE,
            ('B', 'X') => ROCK_SCORE + LOSS_SCORE,
            ('B', 'Y') => PAPER_SCORE + DRAW_SCORE,
            ('B', 'Z') => SCISSORS_SCORE + WIN_SCORE,
            ('C', 'X') => PAPER_SCORE + LOSS_SCORE,
            ('C', 'Y') => SCISSORS_SCORE + DRAW_SCORE,
            ('C', 'Z') => ROCK_SCORE + WIN_SCORE,
            _ => continue,
        };
    }

    println!("{}", score);

    Ok(())
}