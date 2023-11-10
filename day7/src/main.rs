use std::fs;
use std::thread::sleep;

fn main() {
    let mut dir_and_file: Vec<Vec<&str>> = vec![];
    let mut current_dir: Vec<&str> = vec![];

    let binding = fs::read_to_string("input.txt").unwrap();
    for line in binding.lines() {
        if line.contains('$') {
            let command: Vec<&str> = line.split(" ").collect();

            if command[1] == "cd" {
                if command[2] == "/" {
                    current_dir = vec!["/"];
                } else if command[2] == ".." {
                    // this might be wrong
                    if current_dir.len() != 0 {
                        current_dir.pop();
                    }
                } else {
                    current_dir.push(command[2])
                }
            }
        } else {
            let mut already_indexed = false;
            let pos = current_dir.len();
            let mut over_pos = 0;
            for i in dir_and_file.to_vec() {
                if i[0] == current_dir[pos - 1] {
                    already_indexed = true;
                    let mut added = false;
                    for x in i {
                        if x == line {
                            added = true
                        }
                        if !added {
                            dir_and_file[over_pos].push(line);
                            break;
                        }
                    }
                }

                over_pos += 1;
            }
            if !already_indexed {
                let mut to_push: Vec<&str> = vec![];
                to_push.push(current_dir.last().unwrap());
                to_push.push(&line);

                dir_and_file.push(to_push);
            }
        }
    }

    for i in dir_and_file.to_vec() {
        let mut skip = false;
        let mut dir_size = 0;

        dir_size = find_size(dir_and_file.to_vec(), i[0])
    }
}


fn find_size(dir_files: Vec<Vec<&str>>, dir: &str) -> i32 {
    let mut dir_size = 0;

    for i in dir_files {
        let mut skip = false;

        if i[0] == dir {
            for x in i {
                line: Vec<&str> = x.split(" ").collect();
                if skip {
                    if line[0] == "dir" {
                        dir_size += find_size(dir_files.to_vec(), line[1])
                    } else {
                        dir_size += line[0].parse::<i32>().unwrap();
                    }
                } else {
                    skip = true;
                }
            }

        }
    }

    return total_size;
}