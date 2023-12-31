use std::collections::HashMap;
use std::fs;

fn main() {
    let mut dir_list: HashMap<String, Vec<&str>> = HashMap::new();
    let mut current_dir = vec!["main/"];
    let mut line_read = false;
    let mut current_lines: Vec<&str> = vec![];
    // yes i hate this

    let bind_line = fs::read_to_string("input.txt").unwrap();
    for line in bind_line.lines() {
        let word_list: Vec<&str> = line.split(" ").collect();

        if line.contains('$') && line_read {
            line_read = false;
            let string = String::from_iter(current_dir.clone());
            if !dir_list.contains_key(&*string) {
                dir_list.insert(string, current_lines.to_vec());
            }

            current_lines.clear();
        }
        // check if it is a command
        if line.contains('$') && (word_list[1] == "cd") {
            // if it is a cd , we dont care about ls here
            let current_dir_length = current_dir.len();
            match word_list[2] {
                "/" => {
                    current_dir.drain(1..current_dir_length);
                }
                ".." => {
                    if current_dir_length > 0 {
                        current_dir.pop();
                        current_dir.pop();
                    }
                }
                _ => {
                    current_dir.push(word_list[2]);
                    current_dir.push("/")
                }
            }
        }
        if line.contains('$') && (word_list[1] == "ls") {
            line_read = true;
        }

        if !line.contains('$') {
            current_lines.push(line);
        }
    }

    let mut totalt = 0;

    // p 2 code
    let total_used = size_finder(dir_list.clone(), "main/");

    let unused = 70000000 - total_used;

    let to_remove = 30000000 - unused;

    // set this high so the programm that checks will always find smaller
    let mut closesd_found = 70000000;
    // end p2 spesific

    for i in dir_list.keys() {
       println!("{}", i);
        let size = size_finder(dir_list.clone(), i);
        println!("{size}");
        if size <= 100000 {
            totalt += size;
            println!("adding");
        }
        println!("------------------");

        // p2 logic
        if (size >= to_remove) && (size < closesd_found) {
            closesd_found = size;
        }

        // end p2 logic
    }
    println!("p1 answer {}", totalt);
    println!("unused size {unused}");
    println!("to remove {to_remove}");
    println!("smallest possible deletion (p2 answer) {closesd_found}")


}

fn size_finder(dir_list: HashMap<String, Vec<&str>>, path: &str) -> i32 {
    let mut dir_size = 0;
    dbg!(&dir_list);
    if let Some(lines) = dir_list.clone().get(path) {

        for i in dir_list.get(path).unwrap() {
            let line: Vec<&str> = i.split(" ").collect();
            if line[0] == "dir" {
                let p2 = line[1].to_string();
                let new_path = format!("{}{}/", path, p2);
                println!("new path: {new_path}");
                let to_add = size_finder(dir_list.clone(), new_path.as_str());
                println!("{to_add}");
                dir_size += to_add
            } else {
                dir_size += line[0].parse::<i32>().unwrap();
            }
        }
    } else {
        panic!("fucc")
    }
    return dir_size;
}