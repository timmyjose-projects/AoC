use std::collections::{HashMap, VecDeque};
use std::io;

fn main() {
    let input = io::stdin().lines().map(|s| s.unwrap()).collect::<Vec<_>>();
    let mut stacks: HashMap<usize, VecDeque<String>> = HashMap::new();

    for line in input.iter() {
        if line.chars().all(|c| c.is_whitespace() || c.is_digit(10)) {
            continue;
        } else {
            if line.contains("move") {
                let cmd = line.split_whitespace().collect::<Vec<_>>();
                let qty = cmd[1].parse::<usize>().unwrap();
                let from = cmd[3].parse::<usize>().unwrap();
                let to = cmd[5].parse::<usize>().unwrap();

                for _ in 0..qty {
                    let item = stacks.get_mut(&from).unwrap().pop_back().unwrap();
                    stacks.get_mut(&to).unwrap().push_back(item);
                }
            } else {
                let mut k = 1;
                let mut iter = line.chars().peekable();
                while iter.peek().is_some() {
                    let chunk = iter.by_ref().take(4).collect::<String>();
                    let chunk = chunk.trim();
                    if stacks.get(&k).is_none() {
                        stacks.insert(k, VecDeque::new());
                    }

                    if !chunk.is_empty() {
                        stacks
                            .get_mut(&k)
                            .unwrap()
                            .push_front(chunk[1..chunk.len() - 1].to_owned());
                    }
                    k += 1;
                }
            }
        }
    }

    for (k, v) in &stacks {
        println!("{k} => {v:?}");
    }

    let num_stacks = stacks.len();
    let mut res = String::new();

    for k in 1..=num_stacks {
        res.push_str(stacks.get(&k).unwrap().back().unwrap());
    }

    println!("{res}");
}