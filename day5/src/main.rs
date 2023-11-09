use std::fs;

fn main() {
    let mut creates: Vec<Vec<char>> = vec![];

    for _ in 0..9 {
        creates.push(vec![]);
    }

    'wrapper: for line in fs::read_to_string("input.txt").unwrap().lines() {
        let char_list: Vec<char> = line.chars().collect();
        let mut new_char_list: Vec<char> = vec![];

        let mut step: i32 = 3;
        for c in char_list {
            if c.is_numeric() {
                break 'wrapper;
            }
            if step == 4 {
                new_char_list.push(c);
                step = 1;
            } else {
                step += 1;
            }
        }

        let mut index_char = 0;
        for c in new_char_list {
            if c != ' ' {
                creates[index_char].insert(0, c);
            }
            index_char += 1;
        }
    }

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut move_amount  = 0;
        let mut move_from = 0;
        let mut move_to = 0;


        let mut found = 0;
        if line.contains("move") {
            for c in chars {
                if c.is_numeric() {
                    if found == 0 {
                        move_amount = c.to_string().parse().unwrap();
                    }
                    if found == 1 {
                        move_from = c.to_string().parse().unwrap();
                    }
                    if found == 2 {
                        move_to = c.to_string().parse().unwrap();
                    }
                    found += 1;
                }
            }
        }

        for _ in 0..move_amount {
            let length = creates[move_from].len();
           // let tmp_char:char = new_creates[move_from][length];
            println!("{}", length);
        }
    }

}