use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    let mut curr_max = 0;

    let lines = io::stdin().lines();

    for line in lines {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if curr_max > max1 {
                max3 = max2;
                max2 = max1;
                max1 = curr_max;
            } else if curr_max > max2 {
                max3 = max2;
                max2 = curr_max;
            } else if curr_max > max3 {
                max3 = curr_max;
            }
            curr_max = 0;
        } else {
            curr_max += line.parse::<u128>()?;
        }
    }

    println!("{}", max1 + max2 + max3);

    Ok(())
}