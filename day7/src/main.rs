use std::fs;

use day7::{find_smallest_dir_to_delete, parse_input, sum_dirs_sizes_with_limit};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let dirs = parse_input(&input);

    let sum: u128 = sum_dirs_sizes_with_limit(&dirs, 100000);

    println!("Total size of directories: {}", sum);

    let dir_to_delete = find_smallest_dir_to_delete(&dirs, 70000000, 30000000);

    println!(
        "Smallest dir to delete: {}, size: {}",
        dir_to_delete.path,
        dir_to_delete.size()
    );
}
