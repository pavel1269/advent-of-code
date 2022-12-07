use std::{collections::HashMap, vec};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = sum_small_dirs(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = size_to_delete(input);
    return result.to_string();
}

#[derive(Debug)]
struct Directory {
    name: String,
    subdirs: Vec<String>,
    files: Vec<usize>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name: name,
            subdirs: Vec::new(),
            files: Vec::new(),
        }
    }
}

fn size_to_delete(input: &str) -> usize {
    let directories = parse_input(input);
    let current_dir_path = vec!["".to_string()];
    let directory_sizes = get_dir_size(&current_dir_path, &directories);

    let disk_size = 70000000;
    let disk_space_required = 30000000;
    let occupied = directory_sizes[&format_dir_key(&current_dir_path)];
    let mut result = None;
    for (_, size) in directory_sizes.iter() {
        if disk_size - occupied + size >= disk_space_required {
            match result {
                None => result = Some(*size),
                Some(prev_size) => {
                    if *size < prev_size {
                        result = Some(*size);
                    }
                }
            }
        }
    }
    return result.unwrap();
}

fn sum_small_dirs(input: &str) -> usize {
    let directories = parse_input(input);
    let current_dir_path = vec!["".to_string()];
    let directory_sizes = get_dir_size(&current_dir_path, &directories);

    let mut result = 0;
    for (_, size) in directory_sizes.iter() {
        if *size <= 100000 {
            result += size;
        }
    }
    return result;
}

fn get_dir_size(
    path: &Vec<String>,
    directories: &HashMap<String, Directory>,
) -> HashMap<String, usize> {
    let dir_path = format_dir_key(path);
    let dir = &directories[&dir_path];

    let mut directory_sizes = HashMap::new();
    let mut size = 0;
    for subdir in dir.subdirs.iter() {
        let mut subdir_path = path.clone();
        subdir_path.push(subdir.clone());
        let result = get_dir_size(&subdir_path, directories);
        //println!("{:?}", &result);
        directory_sizes.extend(result);
        size += directory_sizes[&format_dir_key(&subdir_path)];
    }

    for file in dir.files.iter() {
        size += file;
    }

    directory_sizes.insert(dir_path, size);

    return directory_sizes;
}

fn parse_input(input: &str) -> HashMap<String, Directory> {
    let root = Directory::new("".to_string());
    let mut directories = HashMap::new();
    let mut current_dir_path = vec![root.name.clone()];
    directories.insert(format_dir_key(&current_dir_path), root);

    let regex_ls = regex::Regex::new(r"^\$ ls$").unwrap();
    let regex_dir_change = regex::Regex::new(r"^\$ cd (\w+)$").unwrap();
    let regex_dir_up = regex::Regex::new(r"^\$ cd ..$").unwrap();
    let regex_file = regex::Regex::new(r"^(\d+) .*$").unwrap();
    let regex_dir = regex::Regex::new(r"^dir (.*)$").unwrap();

    for line in input.lines().skip(1) {
        // println!("{:?} - {:?}", directories, current_dir_path);
        if regex_ls.is_match(line) {
            continue;
        }
        if regex_dir_up.is_match(line) {
            current_dir_path.pop();
            continue;
        }
        match regex_dir_change.captures(line) {
            Some(captures) => {
                let dir_name = captures[1].parse::<String>().unwrap();
                current_dir_path.push(dir_name);
                continue;
            }
            None => {}
        }

        let current_dir = directories.entry(format_dir_key(&current_dir_path));
        match regex_file.captures(line) {
            None => {}
            Some(captures) => {
                let file_size = captures[1].parse().unwrap();
                current_dir.and_modify(|dir| dir.files.push(file_size));
                continue;
            }
        }
        match regex_dir.captures(line) {
            None => {}
            Some(captures) => {
                let dir_name = captures[1].parse::<String>().unwrap();
                current_dir.and_modify(|dir| dir.subdirs.push(dir_name.clone()));
                directories.insert(
                    format_dir_key_new(&current_dir_path, &dir_name),
                    Directory::new(dir_name),
                );
                continue;
            }
        }

        panic!("Failed to parse '{}'", line);
    }

    return directories;
}

fn format_dir_key(path: &Vec<String>) -> String {
    let mut result = String::new();
    for dir in path.iter() {
        result += dir;
        result += "/";
    }
    return result;
}

fn format_dir_key_new(path: &Vec<String>, dir: &String) -> String {
    let mut path_new = path.clone();
    path_new.push(dir.clone());
    return format_dir_key(&path_new);
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "$ cd /
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
7214296 k"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = sum_small_dirs(input);

        assert_eq!(result, 95437);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "1989474");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = size_to_delete(input);

        assert_eq!(result, 24933642);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "1111607");
    }
}
