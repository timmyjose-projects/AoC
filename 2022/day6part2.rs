use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn all_different(slice: &[char]) -> bool {
    for i in 0..slice.len() {
        for j in i + 1..slice.len() {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let input = get_input();
    let mut end = 0;
    let mut pos = 0;
    let mut window = Vec::with_capacity(14);

    while end < input.len() {
        window.push(input.chars().nth(end).unwrap());

        if end >= 13 {
            if all_different(&window) {
                pos = end + 1;
                break;
            }
            window.remove(0);
        }
        end += 1;
    }

    println!("{pos}");
}
