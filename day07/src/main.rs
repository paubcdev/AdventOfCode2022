use std::{collections::HashMap, error::Error};
use parse_display::{Display, FromStr};

struct FileSystem{
    tree: HashMap<String, Directory>
}

impl FileSystem {
    fn dir_size(&self, dir: &String) -> Option<i64> {
        if let Some(dir) = self.tree.get(dir) {
            let file_total = dir.files.iter()
                    .map(|f| f.size)
                    .sum::<i64>();
            let dir_total = dir.children.iter().map(|d| self.dir_size(d).unwrap_or(0)).sum::<i64>();
    
            Some(file_total + dir_total)
        } else {
            None
        }
    }
}

#[derive(Debug,Clone)]
struct Directory {
    files: Vec<FileMetadata>,
    children: Vec<String>
}


#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{size} {name}")]
struct FileMetadata {
    name: String,
    size: i64
}


fn main() -> Result<(), Box<dyn Error>> {
    let data = include_str!("../../inputs/day07.txt");
    
    println!("Part 1: {}", solve_part_1(data));
    println!("Part 2: {}", solve_part_2(data));
    Ok(())
}

fn solve_part_1(data: &str) -> i64 {
    let fs = parse_tree(data);
    let mut result = 0;
    for dir in fs.tree.keys() {
        if let Some(size) = fs.dir_size(dir){
            if size <= 100000 {
                result += size
            }
        }
    }
    result
}

fn solve_part_2(data: &str) -> i64 {
    let fs = parse_tree(data);
    let free_space = 70000000 - fs.dir_size(&String::from("")).unwrap();
    let mut result_2 = i64::MAX;
    for dir in fs.tree.keys() {
        if let Some(size) = fs.dir_size(dir) {
            if free_space + size >= 30000000 {
                result_2 = result_2.min(size)
            }
        }
    }
    result_2
}


fn parse_tree(data:&str)  -> FileSystem {
    let mut dir_stack = vec![];
    let mut tree = HashMap::new();
    for command in data[2..].split("\n$ ") {
        if command == "cd /" {
            dir_stack.clear();
            dir_stack.push("");
        } else if command == "cd .." {
            dir_stack.pop();
        } else if let Some(path) = command.strip_prefix("cd "){
            dir_stack.push(path)
        } else if command.starts_with("ls") {
            let mut results = command.lines();
            results.next();
            let mut children = vec![];
            let mut files = vec![];
            for line in results {
                if let Some(dirname) = line.strip_prefix("dir ") {
                    children.push(format!("{}/{}", dir_stack.join("/"),dirname));
                } else {
                    files.push(line.parse::<FileMetadata>().unwrap());
                }
            }
            tree.insert(dir_stack.join("/"),Directory { files, children });
            
        }
    }
    FileSystem{tree}
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "$ cd /
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
7214296 k
";

    #[test]
    fn test_part_1() {
        let part1 = solve_part_1(DATA);
        assert_eq!(part1, 95437);
    }

    #[test]
    fn test_part_2() {
        let part2 = solve_part_2(DATA);
        assert_eq!(part2, 24933642);
    }
}