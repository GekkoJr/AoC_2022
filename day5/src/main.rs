use std::fs;

fn main() {
    let mut creates: Vec<Vec<char>> = vec![];

    for _ in 0..10 {
        creates.push(vec![]);
    }

    'wrapper: for line in fs::read_to_string("input.txt").unwrap().lines() {
        let char_list: Vec<char> = line.chars().collect();
        let mut new_char_list: Vec<char> = vec![];

        let mut not_empty: bool = true;
        for c in char_list {
            if c.is_numeric() {
                break 'wrapper;
            }
            if !(c == '[' || c == ']') && not_empty {
                new_char_list.push(c);
                not_empty = false;
            } else {
                not_empty = true;
            }
        }

        dbg!(new_char_list);
    }
}