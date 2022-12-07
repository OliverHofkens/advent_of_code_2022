use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;

fn get_input_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::read_to_string(filename).expect("Failed to read file")
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn size(&self) -> u64 {
        let mut size = self.files.iter().fold(0, |acc, f| acc + f.size);
        for sub in &self.subdirs {
            size += sub.borrow().size();
        }
        size
    }

    pub fn flatten(&self) -> Vec<Rc<RefCell<Directory>>> {
        let mut res = self.subdirs.clone();
        for sub in &self.subdirs {
            let _s = sub.borrow();
            res.append(&mut _s.flatten());
        }
        res
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

fn main() {
    let inp = get_input_contents();

    let fs = Rc::new(RefCell::new(Directory {
        name: "/".to_string(),
        files: Vec::new(),
        subdirs: Vec::new(),
        parent: None,
    }));

    let mut working_dir = fs.clone();

    for line in inp.lines() {
        let cmd: Vec<&str> = line.split_whitespace().collect();

        match cmd[0] {
            "$" => match cmd[1] {
                "cd" => match cmd[2] {
                    "/" => working_dir = fs.clone(),
                    ".." => {
                        let next_dir = working_dir.borrow().parent.as_ref().unwrap().clone();
                        working_dir = next_dir
                    }
                    dirname => {
                        let next_dir = working_dir
                            .borrow()
                            .subdirs
                            .iter()
                            .find(|d| d.borrow().name == dirname)
                            .unwrap()
                            .clone();
                        working_dir = next_dir
                    }
                },
                _ => (),
            },
            "dir" => working_dir
                .borrow_mut()
                .subdirs
                .push(Rc::new(RefCell::new(Directory {
                    name: cmd[1].to_string(),
                    files: Vec::new(),
                    subdirs: Vec::new(),
                    parent: Some(working_dir.clone()),
                }))),
            size => working_dir.borrow_mut().files.push(File {
                name: cmd[1].to_string(),
                size: size.parse::<u64>().unwrap(),
            }),
        };
    }

    let flat = fs.borrow().flatten();
    let flat_dirs_with_size: Vec<(u64, _)> = flat.iter().map(|d| (d.borrow().size(), d)).collect();

    let p1: u64 = flat_dirs_with_size
        .iter()
        .filter(|(size, d)| *size <= 100000)
        .map(|(size, d)| size)
        .sum();

    println!("Puzzle 1: {}", p1);

    const total_space: u64 = 70000000;
    const needed_space: u64 = 30000000;
    let space_to_free = needed_space - (total_space - fs.borrow().size());
    let p2 = flat_dirs_with_size
        .iter()
        .filter(|(size, d)| *size >= space_to_free)
        .map(|(size, d)| size)
        .min()
        .unwrap();
    println!("Puzzle 2: {}", p2);
}
