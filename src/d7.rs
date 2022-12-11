use crate::common;
use std::collections::HashMap;

#[derive(Debug)]
struct VFile {
    size: u32,
    sub_files: HashMap<String, VFile>,
}

struct FileSystem {
    root_dir: VFile,
    current_dir: String,
}

impl VFile {
    fn new() -> VFile {
        VFile {
            size: 0,
            sub_files: HashMap::new(),
        }
    }

    fn parse(line: &str) -> (&str, VFile) {
        let (dir_or_size, name) = line.split_once(' ').unwrap();
        let is_dir = dir_or_size == "dir";
        if is_dir {
            (name, VFile::new())
        } else {
            let mut file = VFile::new();
            file.size = dir_or_size.parse::<u32>().unwrap();
            (name, file)
        }
    }

    fn add_file(&mut self, dir: &str, filename: &str, file: VFile) {
        if dir.is_empty() {
            self.sub_files.insert(filename.to_string(), file);
            return;
        }

        let (subdir_name, rest) = dir.split_once('/').unwrap_or((dir, ""));

        self.sub_files
            .get_mut(subdir_name)
            .unwrap()
            .add_file(rest, filename, file);
    }

    fn calc_size(&mut self) {
        for value in self.sub_files.values_mut() {
            value.calc_size();
            self.size += value.size;
        }
    }

    fn get_sizes(&self) -> Vec<u32> {
        let mut sizes = Vec::new();

        let is_folder = !self.sub_files.is_empty();
        if !is_folder {
            return sizes;
        }

        sizes.push(self.size);
        for value in self.sub_files.values() {
            let mut subdirectory_sizes = value.get_sizes();
            sizes.append(&mut subdirectory_sizes);
        }

        sizes
    }
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = VFile::new();
        FileSystem {
            root_dir: root,
            current_dir: "/".to_string(),
        }
    }

    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            let parent_dir = self.current_dir.rfind('/').unwrap();
            self.current_dir.split_off(parent_dir).truncate(0);
            return;
        }
        if dir == "/" {
            self.current_dir = dir.to_string();
            return;
        }

        if !self.current_dir.ends_with('/') {
            self.current_dir += "/";
        }
        self.current_dir += dir;
    }

    fn ls(&mut self, item: &str) {
        let (filename, file) = VFile::parse(item);
        self.root_dir
            .add_file(self.current_dir.split_once('/').unwrap().1, filename, file)
    }
}

fn parse_command(line: &str) -> (&str, &str) {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() == 3 {
        return (parts.get(1).unwrap(), parts.get(2).unwrap());
    } else {
        return (parts.get(1).unwrap(), "");
    }
}

pub fn solve() {
    println!("Day 7:");
    let mut file_system = FileSystem::new();

    let all = common::read_file_to_string("input7.txt".to_string());
    let lines = all.lines();
    for line in lines {
        let is_command = line.starts_with('$');
        if is_command {
            let (command, params) = parse_command(line);
            if command.starts_with("cd") {
                file_system.cd(params);
            }
        } else {
            // not command => Then result of ls
            file_system.ls(line);
        }
    }
    file_system.root_dir.calc_size();

    let first_limit = 100_000;
    let all_directory_sizes = file_system.root_dir.get_sizes();

    let mut sum_under_limit = 0;
    {
        for size in all_directory_sizes.iter() {
            if size < &first_limit {
                sum_under_limit += size;
            }
        }
    }

    println!("sum under limit: {sum_under_limit}");

    let total_disk_size = 70_000_000;
    let needed_disk_space = 30_000_000;

    let used_disk_space = file_system.root_dir.size;
    let free_disk_space = total_disk_size - used_disk_space;
    let need_to_free = needed_disk_space - free_disk_space;

    let mut possible_freed = Vec::new();
    for size in all_directory_sizes {
        if size >= need_to_free {
            possible_freed.push(size);
        }
    }

    possible_freed.sort();
    println!("Smallest dir to delete has size: {}\n", possible_freed[0])
}
