use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FSNode {
    Directory { path: String, children: Vec<FSNode> }, // TODO add indirect children
    File { path: String, size: u128 },
}

impl FSNode {
    fn size(&self) -> u128 {
        match self {
            FSNode::Directory { path: _, children } => {
                let sum = children.iter().map(|c| c.size()).sum();

                if sum > 100000 {
                    0
                } else {
                    sum
                }
            }
            FSNode::File { path: _, size } => *size,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<FSNode> {
    let mut current_path = PathBuf::new();

    let mut fs_map: HashMap<String, FSNode> = HashMap::new();

    let lines = input.lines().collect::<Vec<_>>();
    for line in lines.iter() {
        let split_line: Vec<&str> = line.split(' ').collect();
        println!("{:?}", split_line);

        match split_line[..] {
            ["$", "cd", ".."] => {
                println!("Found cd up one level command");
                current_path.pop();
            }
            ["$", "cd", path] => {
                println!("Found cd to {} command", path);

                current_path.push(path);

                let abs_path = current_path.to_str().unwrap();

                fs_map
                    .entry(abs_path.to_string())
                    .or_insert(FSNode::Directory {
                        path: abs_path.to_string(),
                        children: vec![],
                    });
            }
            ["$", "ls"] => println!("Found list directory contents command"),
            ["dir", dir_name] => {
                println!("Found directory {}", dir_name);

                current_path.push(dir_name);

                let abs_path = current_path.to_str().unwrap();

                fs_map
                    .entry(abs_path.to_string())
                    .or_insert(FSNode::Directory {
                        path: abs_path.to_string(),
                        children: vec![],
                    });

                current_path.pop();
            }
            [size, file_name] => {
                println!("Found file {} with size {}", file_name, size);

                let mut file_name_path = current_path.clone();
                file_name_path.push(file_name);

                let f = FSNode::File {
                    path: file_name_path.to_str().unwrap().to_string(),
                    size: size.parse::<u128>().unwrap(),
                };

                // Add as child of immediate parent
                fs_map
                    .entry(current_path.to_str().unwrap().to_string())
                    .and_modify(|node| {
                        if let FSNode::Directory { path: _, children } = node {
                            children.push(f.clone());
                        }
                    })
                    .or_insert(FSNode::Directory {
                        path: current_path.to_str().unwrap().to_string(),
                        children: vec![f.clone()],
                    });

                // Add as indirect child to all ancestor directories
                let mut traversing_path = current_path.clone();
                while traversing_path.pop() {
                    fs_map
                        .entry(traversing_path.to_str().unwrap().to_string())
                        .and_modify(|node| {
                            if let FSNode::Directory { path: _, children } = node {
                                children.push(f.clone());
                            }
                        })
                        .or_insert(FSNode::Directory {
                            path: traversing_path.to_str().unwrap().to_string(),
                            children: vec![f.clone()],
                        });
                }
            }
            _ => panic!("Unhandled input"),
        }

        println!("Current path: {:#?}", &current_path);
    }

    println!("{:?}", fs_map);

    fs_map.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn get_dir_a() -> FSNode {
        FSNode::Directory {
            path: String::from("a"),
            children: vec![
                FSNode::Directory {
                    path: String::from("e"),
                    children: vec![FSNode::File {
                        path: String::from("i"),
                        size: 584,
                    }],
                },
                FSNode::File {
                    path: String::from("f"),
                    size: 29116,
                },
                FSNode::File {
                    path: String::from("g"),
                    size: 2557,
                },
                FSNode::File {
                    path: String::from("h.lst"),
                    size: 62596,
                },
            ],
        }
    }

    #[test]
    fn calculate_size_a_example() {
        let dir_a = get_dir_a();

        assert_eq!(dir_a.size(), 94853);
    }

    #[test]
    #[ignore]
    fn calculate_size_root_example() {
        let dir_root = FSNode::Directory {
            path: String::from("/"),
            children: vec![
                get_dir_a(),
                FSNode::File {
                    path: String::from("b.txt"),
                    size: 14848514,
                },
                FSNode::File {
                    path: String::from("c.dat"),
                    size: 8504156,
                },
                FSNode::Directory {
                    path: String::from("d"),
                    children: vec![
                        FSNode::File {
                            path: String::from("j"),
                            size: 4060174,
                        },
                        FSNode::File {
                            path: String::from("d.log"),
                            size: 8033020,
                        },
                        FSNode::File {
                            path: String::from("d.ext"),
                            size: 5626152,
                        },
                        FSNode::File {
                            path: String::from("k"),
                            size: 7214296,
                        },
                    ],
                },
            ],
        };

        assert_eq!(dir_root.size(), 48381165);
    }

    #[test]
    fn example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let nodes = parse_input(&input);

        let sum: u128 = nodes.iter().map(|n| n.size()).sum();

        assert_eq!(sum, 95437);
    }
}
