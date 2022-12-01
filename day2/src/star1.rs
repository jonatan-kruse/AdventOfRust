use std::{fs::File, io::Read};

pub fn star1(){
    let mut file = File::open("C:/Users/jonat/progProject/egnaProject/AdventOfRust/day2/src/in_test.txt")
    .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    println!("{}", data)
}