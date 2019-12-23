use std::fs;

fn main() {
    //input file
    const INPUT: &str = include_str!("input_day3.csv");
    let wires: Vec<&str> = INPUT.trim().split("\n").collect();
    let wire1: Vec<&str> = wires[0].split(",").collect();
    let wire2: Vec<&str> = wires[1].split(",").collect();
    println!("input: {:?}", wire1);
    println!("input: {:?}", wire2);


}

