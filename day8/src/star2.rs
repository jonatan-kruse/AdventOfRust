use std::collections::HashSet;

pub fn star2() {
    let _data: Vec<Vec<u32>> = include_str!("in_2.txt")
        .lines()
        .map(|l| l.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    let biggest_top: Vec<u32> = _data[0].clone();
    let mut biggest_left: Vec<u32> = vec![];
    let mut set: HashSet<i32> = HashSet::new();
    _data.iter().for_each(|l| biggest_left.push(l[0]));
    for i in 0..biggest_left.len() {
        for j in 0..biggest_top.len() {
            let d = _data[i][j];
            let mut up = 0;
            let mut down = 0;
            let mut right = 0;
            let mut left = 0;
            let mut a: i32 = 1;
            let mut b: i32 = 1;
            let mut c: i32 = 1;
            let mut e: i32 = 1;
            loop {
                if i as i32 - a < 0 { break;}
                up += 1;
                if _data[(i as i32 - a) as usize][j] >= d { break;}
                a += 1;
            }
            loop {
                if i as i32 + b > (_data.len() - 1).try_into().unwrap() { break;}
                down += 1;
                if _data[(i as i32 + b) as usize][j] >= d { break;}
                b += 1;
            }
            loop {
                if j as i32 - c < 0 { break;}
                left += 1;
                if _data[i][(j as i32 - c) as usize] >= d { break;}
                c += 1;
            }
            loop {
                if j as i32 + e > (_data.len() - 1).try_into().unwrap() { break;}
                right += 1;
                if _data[i][(j as i32 + e) as usize] >= d { break;}
                e += 1;
            }
            //println!("x: {}, y: {}, score: {}", j, i, up * down * right * left);
            set.insert(up * down * right * left);
            
        }
    }
    dbg!(set.iter().reduce(|a, b| if a < b {b} else {a}));
}
