use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn is_top_3_elf(elf: i32, top_3: [i32; 3]) -> [i32; 3] {

    let mut temp = elf;
    let mut temp_list = top_3;
    for i in 0..3 {
        if temp > top_3[i] {
            let new_temp = temp_list[i];
            temp_list[i] = temp;
            temp = new_temp;
        }
    }

    return  temp_list;
}

fn main() {

    let input = lines_from_file("input.txt");

    let mut temp_sum = 0;
    let mut top_elves: [i32; 3] = [0,0,0];
    for line in input {
        if line != "" {
            temp_sum += line.parse::<i32>().unwrap();
        }
        else {
            top_elves = is_top_3_elf(temp_sum, top_elves);
            temp_sum = 0;
        }
    }
    top_elves = is_top_3_elf(temp_sum, top_elves);

    println!("Largest elves are: ");
    for x in top_elves {
        println!("{x}")
    }
    let total: i32 = top_elves.iter().sum();
    println!("{total}")
}
