use std::collections::HashSet;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut score = 0u64;
    let mut cnt = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        cnt += 1;

        match cnt % 3 {
            1 => line.chars().for_each(|c| _ = set1.insert(c)),
            2 => line.chars().for_each(|c| _ = set2.insert(c)),
            _ => {
                for c in line.chars() {
                    if set1.contains(&c) && set2.contains(&c) {
                        score += match c {
                            c if c.is_lowercase() => c as u64 - 'a' as u64 + 1,
                            c => c as u64 - 'A' as u64 + 27,
                        };
                        break;
                    }
                }
                set1.clear();
                set2.clear();
            }
        }
    }

    println!("{score}");

    Ok(())
}