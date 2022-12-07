use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct DirEntry {
    file_sz: usize,
    children: HashMap<String, Rc<RefCell<DirEntry>>>,
    parent: Weak<RefCell<DirEntry>>,
}

impl DirEntry {
    pub fn new() -> Self {
        DirEntry {
            file_sz: 0,
            children: HashMap::new(),
            parent: Weak::new(),
        }
    }
}

fn calculate_sizes(root: Rc<RefCell<DirEntry>>) -> usize {
    let children = root.borrow().children.clone();

    let mut children_sz = 0;
    for (_, sub_dir) in children {
        children_sz += calculate_sizes(Rc::clone(&sub_dir));
    }

    root.borrow_mut().file_sz += children_sz;
    root.borrow().file_sz
}

fn part1(root: Rc<RefCell<DirEntry>>) {
    let mut ans = 0;
    find_small_dirs(root, &mut ans);
    println!("{}", ans);
}

fn find_small_dirs(root: Rc<RefCell<DirEntry>>, ans: &mut usize) {
    if root.borrow().file_sz <= 100_000 {
        *ans += root.borrow().file_sz;
    }

    let children = root.borrow().children.clone();
    for (_, sub_dir) in children {
        let start_dir = Rc::clone(&sub_dir);
        find_small_dirs(start_dir, ans);
    }
}

fn find_spaces(root: Rc<RefCell<DirEntry>>, spaces: &mut Vec<usize>) {
    spaces.push(root.borrow().file_sz);

    let children = root.borrow().children.clone();
    for (_, sub_dir) in children {
        let start_dir = Rc::clone(&sub_dir);
        find_spaces(start_dir, spaces);
    }
}

fn find_min_space(spaces: &[usize], extra_space_needed: usize) -> usize {
    let mut low = 0;
    let mut high = spaces.len() - 1;

    while low < high {
        let mid = low + (high - low) / 2;

        if spaces[mid] < extra_space_needed {
            low = mid + 1;
        } else if spaces[mid] > extra_space_needed {
            high = mid;
        } else {
            return spaces[mid];
        }
    }

    spaces[low]
}

fn part2(root: Rc<RefCell<DirEntry>>) {
    let disk_space = 70000000;
    let needed_space = 30000000;

    let root_space = root.borrow().file_sz;
    let free_space = disk_space - root_space;

    let extra_space_needed = needed_space - free_space;

    let mut spaces = Vec::new();
    find_spaces(root, &mut spaces);
    spaces.sort();
    println!("{}", find_min_space(&spaces, extra_space_needed));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut root: Rc<RefCell<DirEntry>> = Rc::new(RefCell::new(DirEntry::new()));
    let mut curr_dir: Rc<RefCell<DirEntry>> = Rc::new(RefCell::new(DirEntry::new()));

    for line in io::stdin().lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with("$") {
            let cmd = line[2..].split_whitespace().collect::<Vec<_>>();

            match cmd[0] {
                "cd" => {
                    let dir = cmd[1];

                    match dir {
                        "/" => {
                            root = Rc::new(RefCell::new(DirEntry::new()));
                            curr_dir = Rc::clone(&root);
                        }

                        ".." => {
                            let new_dir = curr_dir
                                .borrow()
                                .parent
                                .upgrade()
                                .ok_or(format!("{}", "could not switch to parent entry"))?;
                            curr_dir = new_dir;
                        }

                        _ => {
                            let new_dir = Rc::clone(
                                curr_dir
                                    .borrow()
                                    .children
                                    .get(dir)
                                    .ok_or(format!("{}", "could not get child entry"))?,
                            );
                            curr_dir = new_dir;
                        }
                    }
                }
                "ls" => {}

                _ => panic!("{}", "unknown command: {cmd:?}"),
            }
        } else {
            let mut tot_file_sz = 0;
            let stats = line.split_whitespace().collect::<Vec<_>>();
            let sub_dir_name = stats[1].to_owned();

            if stats[0] == "dir" {
                let sub_dir = Rc::new(RefCell::new(DirEntry::new()));
                sub_dir.borrow_mut().parent = Rc::downgrade(&curr_dir);
                curr_dir.borrow_mut().children.insert(sub_dir_name, sub_dir);
            } else {
                tot_file_sz += stats[0].parse::<usize>()?;
            }

            curr_dir.borrow_mut().file_sz += tot_file_sz;
        }
    }

    calculate_sizes(Rc::clone(&root));

    part1(Rc::clone(&root));
    part2(Rc::clone(&root));

    Ok(())
}