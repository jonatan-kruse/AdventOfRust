use std::collections::HashSet;

pub fn star2() {
    let mut _data = include_str!("in_2.txt").chars();
    let mut possible_marker: Vec<char> = _data.clone().take(14).collect();
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
    dbg!(include_str!("in_2.txt").len() - _data.count());
}
