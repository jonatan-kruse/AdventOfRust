use std::collections::HashSet;

pub fn star1() {
    let _data: Vec<Vec<u32>> = include_str!("in_2.txt")
        .lines()
        .map(|l| l.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    let mut biggest_top: Vec<u32> = _data[0].clone();
    let mut biggest_left: Vec<u32> = vec![];
    let mut set: HashSet<(usize, usize , u32)> = HashSet::new();
    _data.iter().for_each(|l| biggest_left.push(l[0]));
    for i in 0..biggest_left.len() {
        for j in 0..biggest_top.len() {
            if i == 0 || j == 0 || i == biggest_left.len() - 1 || j == biggest_top.len() - 1 {
                let d = _data[i][j];
                set.insert((j, i, d));
            }
        }
    }
    for i in 0..biggest_left.len() {
        for j in 0..biggest_top.len() {
            let d = _data[i][j];
            if d > biggest_left[i] {
                set.insert((j, i, d));
                biggest_left[i] = d;
            }
            if d > biggest_top[j] {
                set.insert((j, i, d));
                biggest_top[j] = d;
            }
        }
    }
    let mut biggest_bottom: Vec<u32> = _data.last().unwrap().clone();
    let mut biggest_right: Vec<u32> = vec![];
    _data.iter().for_each(|l| biggest_right.push(*l.last().unwrap()));
    for i in 0..biggest_right.len() {
        for j in 0..biggest_bottom.len() {
            let x = biggest_bottom.len() - j - 1;
            let y = biggest_right.len() - i - 1;
            let d = _data[y][x];
            if d > biggest_right[y] {
                set.insert((x, y, d));
                biggest_right[y] = d;
            }
            if d > biggest_bottom[x] {
                set.insert((x, y, d));
                biggest_bottom[x] = d;
            }
        }
    }
    dbg!(set.len());
}
