use std::fs;

fn main() {
    let file = "input.txt";
    let data = parse(file);

    let result = find_visible(data);

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
                        println!("left")
                    }
                    highest_check = 0
                }
                println!("current highest {highest_check}")
            }
            if compare(highest_check, current_num) {
                visible = true;
                println!("right")
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
                        println!("top")
                    }
                    highest_check = 0
                }
            }
            if compare(highest_check, data[cc][cl]) {
                visible = true;
                println!("bottom")
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