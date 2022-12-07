use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut max_so_far = 0;
    let mut curr_max = 0;

    let lines = io::stdin().lines();

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if curr_max > max_so_far {
                max_so_far = curr_max;
            }
            curr_max = 0;
        } else {
            curr_max += line.parse::<u128>()?;
        }
    }
    println!("{}", max_so_far);

    Ok(())
}