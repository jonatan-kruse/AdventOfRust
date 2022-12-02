use std::collections::HashMap;

pub fn star2(){
    let data:Vec<&str> = include_str!("in_2.txt").lines().collect();
    let my_map:HashMap<&str, i32> = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6)
    ]);
    let value: i32 = data.into_iter().map(|x| my_map.get(x).unwrap()).sum();
    println!("{}", value)
}