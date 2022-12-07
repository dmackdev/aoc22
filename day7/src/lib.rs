use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Directory {
    pub path: String,
    files: Vec<Rc<File>>,
    indirect_files: Vec<Rc<File>>,
}

#[derive(Debug, Clone)]
pub struct File {
    path: String,
    size: u128,
}

impl Directory {
    pub fn size(&self) -> u128 {
        self.files
            .iter()
            .chain(self.indirect_files.iter())
            .map(|f| f.size)
            .sum()
    }
}

pub fn parse_input(input: &str) -> Vec<Directory> {
    let mut current_path = PathBuf::new();

    let mut fs_map: HashMap<String, Directory> = HashMap::new();

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

                fs_map.entry(abs_path.to_string()).or_insert(Directory {
                    path: abs_path.to_string(),
                    files: vec![],
                    indirect_files: vec![],
                });
            }
            ["$", "ls"] => println!("Found list directory contents command"),
            ["dir", dir_name] => {
                println!("Found directory {}", dir_name);

                current_path.push(dir_name);

                let abs_path = current_path.to_str().unwrap();

                fs_map.entry(abs_path.to_string()).or_insert(Directory {
                    path: abs_path.to_string(),
                    files: vec![],
                    indirect_files: vec![],
                });

                current_path.pop();
            }
            [size, file_name] => {
                println!("Found file {} with size {}", file_name, size);

                let mut file_name_path = current_path.clone();
                file_name_path.push(file_name);

                let f = Rc::new(File {
                    path: file_name_path.to_str().unwrap().to_string(),
                    size: size.parse::<u128>().unwrap(),
                });

                // Add as child of immediate parent
                fs_map
                    .entry(current_path.to_str().unwrap().to_string())
                    .and_modify(|directory| {
                        directory.files.push(f.clone());
                    })
                    .or_insert(Directory {
                        path: current_path.to_str().unwrap().to_string(),
                        files: vec![f.clone()],
                        indirect_files: vec![],
                    });

                // Add as indirect child to all ancestor directories
                let mut traversing_path = current_path.clone();
                while traversing_path.pop() {
                    fs_map
                        .entry(traversing_path.to_str().unwrap().to_string())
                        .and_modify(|directory| {
                            directory.indirect_files.push(f.clone());
                        })
                        .or_insert(Directory {
                            path: traversing_path.to_str().unwrap().to_string(),
                            files: vec![],
                            indirect_files: vec![f.clone()],
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

pub fn sum_dirs_sizes_with_limit(dirs: &[Directory], limit: u128) -> u128 {
    dirs.iter()
        .filter_map(|n| {
            let size = n.size();
            if size > limit {
                None
            } else {
                Some(size)
            }
        })
        .sum()
}

pub fn find_smallest_dir_to_delete(
    dirs: &[Directory],
    max_fs_space: u128,
    required_space: u128,
) -> &Directory {
    let root_dir = dirs.iter().find(|dir| dir.path == "/").unwrap();
    let total_used_space = root_dir.size();
    let total_unused_space = max_fs_space - total_used_space;
    let total_space_to_delete = required_space - total_unused_space;

    let mut possible_dirs: Vec<_> = dirs
        .iter()
        .filter_map(|dir| {
            let size = dir.size();
            if size >= total_space_to_delete {
                Some((size, dir))
            } else {
                None
            }
        })
        .collect();

    possible_dirs.sort_by_key(|(size, _)| *size);

    possible_dirs.first().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example() {
        let input =
            fs::read_to_string("test_input.txt").expect("Should have been able to read the file");

        let dirs = parse_input(&input);

        let sum: u128 = sum_dirs_sizes_with_limit(&dirs, 100000);

        assert_eq!(sum, 95437);

        let dir_to_delete = find_smallest_dir_to_delete(&dirs, 70000000, 30000000);

        assert_eq!(dir_to_delete.path, "/d");
    }
}
