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
        let mut move_amount = 0;
        let mut move_from = 0;
        let mut move_to = 0;


        if line.contains("move") {
            let line_to_mod = line;
            let mut found = 0;
            for x in line_to_mod.split_whitespace() {
                if x.parse::<i32>().is_ok() {
                    if found == 0 {
                        move_amount = x.parse().unwrap();
                    }
                    if found == 1 {
                        move_from = x.parse().unwrap()
                    }
                    if found == 2 {
                        move_to = x.parse().unwrap();
                    }
                    found += 1;
                }
            }
            let mut tmp_vec = vec![];
            for _ in 0..move_amount {
                let length = creates[move_from - 1].len() - 1;
                println!("{}", length);
                let tmp_char: char = creates[move_from - 1][length ];
                creates[move_from - 1].remove(length);
                tmp_vec.insert(0, tmp_char);
                // creates[move_to - 1].push(tmp_char);
            }
            creates[move_to -1].append(&mut tmp_vec);

        }



    }

    for letter in creates {
        println!("{}", letter[letter.len() - 1])
    }
}