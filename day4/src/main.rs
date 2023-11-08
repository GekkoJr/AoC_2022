use std::fs;

fn main() {
    let mut total: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let elves: Vec<&str> = line.split(',').collect();
        let elf_a: Vec<&str> = elves[0].split("-").collect();
        let elf_b: Vec<&str> = elves[1].split("-").collect();


        let a: i32 = elf_a[0].parse().unwrap();
        let b: i32 = elf_a[1].parse().unwrap();
        let c: i32 = elf_b[0].parse().unwrap();
        let d: i32 = elf_b[1].parse().unwrap();

        if ((a <= c && c <= b) || (a <= d && d <= b))  || ((c <= a && a <= d) || (c <= b && b <= d)) {
            total += 1;
        }
    }

    println!("{}", total)
}