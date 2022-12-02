use std::collections::HashMap;

pub fn star1(){
    let data:Vec<&str> = include_str!("in_2.txt").lines().collect();
    let my_map:HashMap<&str, i32> = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3)
    ]);
    let value: i32 = data.into_iter().map(|x| my_map.get(x).unwrap()).sum();
    println!("{}", value)
}