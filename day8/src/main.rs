use std::fs;

fn main() {
    let file = "input.txt";
    let data = parse(file);

    let result = find_visible(data.clone());
    let p2 = find_score(data.clone());
    println!("-------------------");
    println!("highest score {p2}");

    println!("total visible: {result}")
}

fn parse(file: &str) -> Vec<Vec<i32>> {
    let rows = fs::read_to_string(file).unwrap();
    let mut data: Vec<Vec<i32>> = vec![];

    let mut index = 0;
    let mut first_pass = true;
    for i in rows.lines() {
        if first_pass {
            for _x in i.chars() {
                data.push(vec![])
            }
            first_pass = false;
        }

        for x in i.chars() {
            data[index].push(x.to_string().parse::<i32>().unwrap())
        }
        index += 1;
    }

    return data;
}

fn find_visible(data: Vec<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;

    // finding the border
    total += (data.len().to_string().parse::<i32>().unwrap()) * 2;
    total += (data[0].len().to_string().parse::<i32>().unwrap() - 2) * 2;

    // going through every line except last and first
    for cc in 1..data.len() - 1 {
        // each num
        for cl in 1..data[cc].len() - 1 {
            let current_num = data[cc][cl];
            println!("current num check: {current_num}");
            let mut visible = false;
            let mut highest_check = 0;

            // checking current line
            for lc in 0..data[cc].len() {
                if lc < cl {
                    if highest_check < data[cc][lc] {
                        highest_check = data[cc][lc];
                    }
                } else if cl < lc {
                    if highest_check < data[cc][lc] {
                        highest_check = data[cc][lc];
                    }
                } else if cl == lc {
                    if compare(highest_check, current_num) {
                        visible = true;
                    }
                    highest_check = 0
                }
            }
            if compare(highest_check, current_num) {
                visible = true;
            }
            highest_check = 0;
            // checking colum
            for rc in 0..data.len() {
                if rc < cc {
                    if highest_check < data[rc][cl] {
                        highest_check = data[rc][cl];
                    }
                } else if cc < rc {
                    if highest_check < data[rc][cl] {
                        highest_check = data[rc][cl];
                    }
                } else if cc == rc {
                    if compare(highest_check, data[cc][cl]) {
                        visible = true;
                    }
                    highest_check = 0
                }
            }
            if compare(highest_check, data[cc][cl]) {
                visible = true;
            }
            if visible {
                total += 1;
                println!("visible {cc}-{cl}")
            }
        }
    }
    return total;
}

fn compare(high: i32, x: i32) -> bool {
    println!("{high} > {x}");
    if high >= x {
        return false;
    } else {
        return true;
    }

}

fn find_score(data: Vec<Vec<i32>>) -> i32 {
    let mut highest_score: i32 = 0;

    let mut col_index = 0;
    for colum in data.clone() {
        let mut num_index =0;
        for num in &colum {
            let mut top = 0;
            let mut bottom = 0;
            let mut left = 0;
            let mut right = 0;

            // left
            for i in (0..num_index).rev() {
                left += 1;
                if colum[i] >= *num {
                    break;
                }

            }

            for i in (num_index+1)..colum.len() {
                right += 1;
                if colum[i] >= *num {
                    break;
                }
            }

            for i in (0..col_index).rev() {
                top += 1;
                if data[i][num_index] >= *num {
                    break;
                }
            }

            for i in (col_index+1)..data.len() {
                bottom += 1;
                if data[i][num_index] >= *num {
                    break;
                }

            }

            let score: i32 = (top * bottom * left * right).to_string().parse::<i32>().unwrap();

            if score.to_string().parse::<i32>().unwrap() > highest_score {
                highest_score = score;
                println!("left {left} right {right} top {top} bottom {bottom}")
            }
            num_index += 1;
        }
        col_index += 1;
    }


    return highest_score;
}