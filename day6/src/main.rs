use std::fs;

fn main() {
    let line = fs::read_to_string("test.txt").unwrap();

    let mut current_chars: Vec<char> = vec![];
    let mut num = 0;

    for i in line.chars() {
        num += 1;
        if num <= 4 {
            current_chars.push(i);
            continue;
        } else {
            current_chars.remove(0);
            let tmp = current_chars.to_vec();
            dbg!(tmp);


            if !current_chars.contains(&i)
            {
                println!("{}", num);
                break;
            }

            current_chars.push(i);
        }
    }
}