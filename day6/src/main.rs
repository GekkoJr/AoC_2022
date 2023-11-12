use std::fs;

fn main() {
    let line = fs::read_to_string("input.txt.txt").unwrap();

    let mut current_chars: Vec<char> = vec![];
    let mut num = 0;
    let mut total = 0;

    for i in line.chars() {
        num += 1;
        total += 1;
        if num < 14 {

            if current_chars.contains(&i) {
                for x in 0..current_chars.len() {
                    if i == current_chars[x] {
                        for _ in 0..x+1 {
                            current_chars.remove(0);
                        }
                        num -= x+1;

                        break
                    }
                }
            }
            current_chars.push(i);
        } else {

            let tmp = current_chars.to_vec();
            dbg!(tmp);


            if !current_chars.contains(&i)
            {
                println!("{}", total);
                break;
            } else  {
                for x in 0..current_chars.len() {
                    if i == current_chars[x] {
                        for _ in 0..x+1 {
                            current_chars.remove(0);
                        }
                        num -= x+1;

                        break
                    }
                }
            }
            current_chars.push(i);
        }
    }
}