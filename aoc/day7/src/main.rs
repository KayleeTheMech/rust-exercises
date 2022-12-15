use std::{fs, iter::Peekable, path::Path, vec};

struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

struct Directory {
    root: bool,
    name: String,
    subdirectories: Vec<Directory>,
    content: Vec<File>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subdirectories: vec![],
            content: vec![],
            root: false,
        }
    }

    fn new_root() -> Self {
        Self {
            name: "/".to_string(),
            subdirectories: vec![],
            content: vec![],
            root: true,
        }
    }

    fn size(&self) -> u32 {
        let mut size: u32 = 0;
        for dir in self.subdirectories.as_slice() {
            size += dir.size();
        }
        for file in self.content.as_slice() {
            size += file.size;
        }

        size
    }

    fn add_dir(&mut self, dir: Directory) {
        if dir.root {
            panic!("Can't add root to other dir.")
        }
        self.subdirectories.push(dir);
    }

    fn add_dir_to_path(&mut self, path: &str, directory: Directory) -> Option<bool> {
        let dir = self.get_path(path)?;
        dir.add_dir(directory);
        Some(true)
    }
    fn add_file_to_path(&mut self, path: &str, file: File) -> Option<bool> {
        let dir = self.get_path(path)?;
        dir.add_file(file);
        Some(true)
    }

    fn add_file(&mut self, file: File) {
        self.content.push(file);
    }

    fn get_subdir(&mut self, name: &str) -> Option<&mut Directory> {
        for dir in self.subdirectories.as_mut_slice() {
            if dir.name == name {
                return Some(dir);
            }
        }
        None
    }

    fn get_path(&mut self, path: &str) -> Option<&mut Directory> {
        let subpath: &str;
        if self.root {
            subpath = path.trim_start_matches(&self.name)
        } else {
            let trim_off = "".to_owned() + &self.name;
            subpath = path.trim_start_matches(&trim_off).trim_start_matches("/");
        }

        match subpath {
            "" | "/" => return Some(self),
            _ => {
                let next_level = self.get_subdir(subpath.split("/").next()?);
                return Some(next_level?.get_path(subpath)?);
            }
        }
    }

    fn get_all_sub_directories(&self) -> Vec<&Directory> {
        let mut all_dirs: Vec<&Directory> = vec![];
        for directory in self.subdirectories.as_slice() {
            all_dirs.append(&mut vec![&directory]);
            all_dirs.append(&mut directory.get_all_sub_directories())
        }
        all_dirs
    }
}

#[test]
fn test_size_calculation() {
    let mut root = Directory::new_root();
    root.add_file_to_path("/", File::new("test1", 4096))
        .expect("Expect success");
    root.add_dir_to_path("/", Directory::new("a"))
        .expect("Expect success");
    root.add_file_to_path("/a", File::new("test2", 4096))
        .expect("Expect success");
    root.add_dir_to_path("/a", Directory::new("b"))
        .expect("Expect success");
    root.add_file_to_path("/a/b", File::new("test3", 4096))
        .expect("Expect success");
    root.add_file(File::new("test4", 4096));
    assert_eq!(root.size(), 4 * 4096);
    let a = root.get_path("/a").expect("expect a to be there");
    assert_eq!(a.size(), 2 * 4096);
}

fn modify_root(root: &mut Directory, working_directory: &str, entry: &str) {
    let mut entry = entry.split(" ");
    let first = entry
        .next()
        .expect("Expect there to be a first element in every ls entry.");
    let second = entry
        .next()
        .expect("Expect there to be a second element in every ls entry.");
    match first {
        "dir" => {
            // create a directory
            root.add_dir_to_path(working_directory, Directory::new(second))
                .expect("It should be possible to add this directory.");
        }
        _ => {
            // is a file, first is size
            let size = first.parse::<u32>().expect("Expect size to be parseable.");
            root.add_file_to_path(working_directory, File::new(second, size))
                .expect("It should be possible to add this File.");
        }
    }
}

fn change_directory<'a>(working_dir: &'a str, target_dir: &'a str) -> String {
    if '/'
        == target_dir
            .chars()
            .next()
            .expect("Expect at least one char as target dir!")
    {
        // Absolute path
        return target_dir.to_string();
    } else {
        // Relative path
        let mut temp: Vec<&str> = working_dir.split("/").collect();
        for dir in target_dir.split("/") {
            match dir {
                ".." => {
                    temp.pop();
                }
                _ => temp.push(dir),
            }
        }
        return "/".to_owned() + &temp.join("/");
    }
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    let input: Vec<&str> = content.trim_end_matches("\n").split("\n").collect();

    let mut root = Directory::new_root();

    let mut working_directory: String = "/".to_owned();
    let mut loop_iterator = input.iter().peekable();
    loop {
        let next = loop_iterator.next();
        let line = match next {
            None => break,
            Some(line) => line.clone(),
        };
        // only process commands at this stage output to act on needs to be processed
        if line
            .chars()
            .next()
            .expect("Expect at least one character per line!")
            != '$'
        {
            panic!("Expecting only commands in this loop!")
        }
        let mut command = line.trim_start_matches("$ ").split(" ");
        match command.next() {
            Some("cd") => {
                working_directory = change_directory(
                    &mut working_directory,
                    command.next().expect("cd requires an additional argument"),
                )
            }
            Some("ls") => {
                // next lines are results from a command, enter readout loop
                let mut content: Vec<&str> = vec![];
                while let Some(&&next) = loop_iterator.peek() {
                    if '$' == next.chars().next().unwrap() {
                        break; // next line is a command; end readout mode
                    }
                    content.append(&mut vec![loop_iterator.next().unwrap()])
                }
                for entry in content {
                    modify_root(&mut root, &working_directory, entry);
                }
            }
            None | _ => {} // ignore unknown commands
        }
    }
    let mut all_above_100k: u32 = 0;

    for directory in root.get_all_sub_directories().as_slice() {
        println!("name: {}, size: {}", directory.name, directory.size());
        if directory.size() > 100000 {
            all_above_100k += directory.size();
            println!("HERE!");
        }
    }

    println!("hello root size is {}", root.size());
    println!("All above 100k: {}", all_above_100k);
}
