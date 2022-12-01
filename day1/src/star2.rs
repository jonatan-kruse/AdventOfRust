use std::{fs::File, io::Read};

pub fn star2() {
  let mut file = File::open("C:/Users/jonat/progProject/egnaProject/AdventOfRust/day1/src/in.txt")
    .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    let elves: Vec<&str> = data.split("\n").collect();
    let mut num = 0;
    let mut lista:Vec<i32> = Vec::new();
    for i in elves {
        if i.trim() == "" {
            lista.push(num);
            num = 0;
        } else {
            num += i.trim().parse::<i32>().unwrap();
        }
    }
    lista.sort();
    

    println!("{}", lista.pop().unwrap() + lista.pop().unwrap() + lista.pop().unwrap());
}