use std::collections::HashSet;

pub fn star2() {
    let mut _data = include_str!("in_2.txt").chars();
    let mut possible_marker: Vec<char> = vec![];
    for _i in 0..14 {
        possible_marker.push(_data.next().unwrap())
    }
    while possible_marker
        .clone()
        .into_iter()
        .collect::<HashSet<char>>()
        .len()
        < 14
    {
        possible_marker.remove(0);
        possible_marker.push(_data.next().unwrap());
    }
    let binding = possible_marker.iter().collect::<String>();
    let strin = binding.as_str();
    dbg!(strin);
    dbg!(include_str!("in_2.txt").find(strin).unwrap() + 14);
}
