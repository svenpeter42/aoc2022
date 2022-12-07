#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::cell::RefCell;
use std::cmp;
use std::error::Error;
use std::fs;
use std::rc::Rc;

struct DirNode {
    name: String,
    size: usize,
    children: Vec<Rc<RefCell<FSNode>>>,
    parent: Option<Rc<RefCell<FSNode>>>,
}

struct FileNode {
    name: String,
    size: usize,
    parent: Option<Rc<RefCell<FSNode>>>,
}

impl DirNode {
    fn find_child(&self, name: &str) -> Option<&Rc<RefCell<FSNode>>> {
        self.children
            .iter()
            .filter(|&c| match &*c.borrow_mut() {
                FSNode::Dir(d) => d.name == name,
                FSNode::File(f) => f.name == name,
            })
            .nth(0)
    }
}

enum FSNode {
    Dir(DirNode),
    File(FileNode),
}

impl FSNode {
    fn get_size(&self) -> usize {
        match self {
            FSNode::Dir(d) => d.size,
            FSNode::File(f) => f.size,
        }
    }

    fn add_size(&mut self, size: usize) {
        match self {
            FSNode::Dir(d) => d.size += size,
            FSNode::File(f) => f.size += size,
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<FSNode>>> {
        match self {
            FSNode::Dir(d) => match d.parent.as_ref() {
                Some(n) => Some(Rc::clone(n)),
                None => None,
            },
            FSNode::File(d) => match d.parent.as_ref() {
                Some(n) => Some(Rc::clone(n)),
                None => None,
            },
        }
    }

    fn find_child(&self, name: &str) -> Option<&Rc<RefCell<FSNode>>> {
        match self {
            FSNode::Dir(d) => d.find_child(name),
            FSNode::File(_) => None,
        }
    }

    fn add_child(&mut self, child: Rc<RefCell<FSNode>>) -> Result<(), &'static str> {
        match self {
            FSNode::Dir(d) => {
                d.children.push(child);
                Ok(())
            }
            FSNode::File(_) => Err("Cannot add children to FileNode"),
        }
    }
}

fn parse_fs(input: &str) -> Result<Rc<RefCell<FSNode>>, &'static str> {
    let mut lines = input.lines();
    match lines.next() {
        Some("$ cd /") => (),
        Some(_) => return Err("Invalid first line"),
        None => return Err("No first line"),
    }

    let root = Rc::new(RefCell::new(FSNode::Dir(DirNode {
        name: "/".to_string(),
        size: 0,
        children: vec![],
        parent: None,
    })));
    let mut current = Rc::clone(&root);

    for line in lines {
        if line == "$ ls" {
            // just assume the input is always valid...
        } else if line == "$ cd .." {
            let parent = current.borrow().get_parent().unwrap();
            parent.borrow_mut().add_size(current.borrow().get_size());
            current = parent;
        } else if line.starts_with("$ cd") {
            let child = Rc::clone(
                current
                    .borrow()
                    .find_child(
                        line.split(" ")
                            .last()
                            .ok_or("$ cd with empty name in input")?,
                    )
                    .ok_or("cd to unknown child")?,
            );
            current = child;
        } else if line.starts_with("dir") {
            current
                .borrow_mut()
                .add_child(Rc::new(RefCell::new(FSNode::Dir(DirNode {
                    name: line
                        .split(" ")
                        .last()
                        .ok_or("dir with empty name in input")?
                        .to_string(),
                    size: 0,
                    children: vec![],
                    parent: Some(Rc::clone(&current)),
                }))))?;
        } else {
            let (size, name) = line
                .split(" ")
                .collect_tuple::<(_, _)>()
                .map(|(size, name)| (size.parse::<usize>().unwrap(), name))
                .unwrap();

            current
                .borrow_mut()
                .add_child(Rc::new(RefCell::new(FSNode::File(FileNode {
                    name: name.to_string(),
                    size: size,
                    parent: Some(Rc::clone(&current)),
                }))))?;
            current.borrow_mut().add_size(size);
        }
    }

    loop {
        let parent = current.borrow().get_parent();

        if let Some(p) = parent {
            p.borrow_mut().add_size(current.borrow().get_size());
            current = p;
        } else {
            break;
        }
    }

    Ok(root)
}

fn calc_small_directories_sum(input: &str) -> Result<usize, &'static str> {
    let fs = parse_fs(input)?;
    let mut queue: Vec<Rc<RefCell<FSNode>>> = vec![];
    let mut size = 0;

    queue.push(fs);
    while let Some(node) = queue.pop() {
        match &*node.borrow() {
            FSNode::Dir(d) => {
                if d.size < 100000 {
                    size += d.size;
                }
                d.children.iter().for_each(|c| queue.push(Rc::clone(c)));
            }
            FSNode::File(_) => (),
        };
    }

    Ok(size)
}

fn find_small_directory_above(input: &str) -> Result<usize, &'static str> {
    let fs = parse_fs(input)?;
    let mut queue: Vec<Rc<RefCell<FSNode>>> = vec![];
    let mut result_size: Option<usize> = None;
    let root_size = fs.borrow().get_size();
    let min_size = 30000000 - (70000000 - root_size);

    queue.push(fs);
    while let Some(node) = queue.pop() {
        match &*node.borrow_mut() {
            FSNode::Dir(d) => {
                if d.size >= min_size {
                    result_size = Some(match result_size {
                        Some(s) => cmp::min(s, d.size),
                        None => d.size,
                    });
                }
                d.children.iter().for_each(|c| queue.push(Rc::clone(c)));
            }
            FSNode::File(_) => (),
        };
    }

    result_size.ok_or("No directory found that can be deleted to reach enough free space")
}

fn main() -> Result<(), Box<dyn Error>> {
    let size = calc_small_directories_sum(&fs::read_to_string("input.txt")?)?;
    let size2 = find_small_directory_above(&fs::read_to_string("input.txt")?)?;
    println!("Hello, world!, {} {}", size, size2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn task_a() {
        assert_eq!(calc_small_directories_sum(EXAMPLE_DATA).unwrap(), 95437);
    }

    #[test]
    fn task_b() {
        assert_eq!(find_small_directory_above(EXAMPLE_DATA).unwrap(), 24933642);
    }
}
