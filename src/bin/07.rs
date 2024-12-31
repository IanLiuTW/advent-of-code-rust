use bisection::bisect_left;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let fs = parse_input(input);

    let mut ans = 0;
    let dir_sizes = fs.get_dir_sizes();

    for size in dir_sizes {
        if size <= 100000 {
            ans += size;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let fs = parse_input(input);

    let mut dir_sizes = fs.get_dir_sizes();
    dir_sizes.sort();

    let needed_size = dir_sizes.last().unwrap() - 40000000;
    let i = bisect_left(&dir_sizes, &needed_size);

    Some(dir_sizes[i])
}

struct Dir {
    id: String,
    parent: Option<Rc<RefCell<Dir>>>,
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, u32>,
}

impl Dir {
    fn new(id: String, parent: Option<Rc<RefCell<Dir>>>) -> Self {
        Self {
            id,
            parent,
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

struct FileSystem {
    root: Rc<RefCell<Dir>>,
    cd: Rc<RefCell<Dir>>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Dir::new("/".to_string(), None)));
        let cd = root.clone();
        Self { root, cd }
    }

    fn cd(&mut self, dir_id: String) {
        if dir_id == "/" {
            self.cd = self.root.clone();
        } else if dir_id == ".." {
            let parent = self.cd.borrow().parent.clone();

            if let Some(parent) = parent {
                self.cd = parent;
            }
        } else {
            let new_dir = self
                .cd
                .borrow_mut()
                .dirs
                .entry(dir_id.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new(Dir::new(
                        dir_id.clone(),
                        Some(self.cd.clone()),
                    )))
                })
                .clone();

            self.cd = new_dir;
        }
    }

    fn add_file(&self, file: (&str, &str)) {
        self.cd
            .borrow_mut()
            .files
            .insert(file.1.to_string(), file.0.parse::<u32>().unwrap());
    }

    fn get_dir_sizes(&self) -> Vec<u32> {
        fn dfs(dir: Rc<RefCell<Dir>>, _dir_sizes: &mut Vec<u32>) -> u32 {
            let dir = dir.borrow();

            let mut size = dir.files.values().sum();
            for child in dir.dirs.values() {
                size += dfs(child.clone(), _dir_sizes);
            }
            _dir_sizes.push(size);

            size
        }

        let mut dir_sizes = vec![];

        dfs(self.root.clone(), &mut dir_sizes);

        dir_sizes
    }
}

fn parse_input(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();

    input.lines().for_each(|line| match line {
        line if line.starts_with("$ cd") => {
            let id = line.trim_start_matches("$ cd ");
            fs.cd(id.to_string());
        }
        line if line.starts_with("$ ls") => {}
        line if line.starts_with("dir") => {}
        _ => {
            let file = line.split_once(" ").unwrap();
            fs.add_file(file);
        }
    });

    fs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24933642));
    }
}
