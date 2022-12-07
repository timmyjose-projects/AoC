use std::collections::HashSet;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut score = 0u64;

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let len = line.len();

        line.chars().take(len / 2).for_each(|c| _ = set1.insert(c));
        line.chars().skip(len / 2).for_each(|c| _ = set2.insert(c));

        let c = set1.intersection(&set2).collect::<Vec<_>>()[0];

        score += match c {
            c if c.is_lowercase() => *c as u64 - 'a' as u64 + 1,
            _ => *c as u64 - 'A' as u64 + 27,
        };

        set1.clear();
        set2.clear();
    }

    println!("{score}");

    Ok(())
}