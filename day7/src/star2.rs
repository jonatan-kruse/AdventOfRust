use std::{borrow::BorrowMut, str::Split};

pub fn star2() {
    let mut _data = include_str!("in_2.txt").split("\n");
    let mut sizes: Vec<i32> = vec![];
    fn evaluate(lines: &mut Split<&str>, sizes: &mut Vec<i32>) -> i32 {
        let mut temp = 0;
        while let Some(l) = lines.next() {
            if l.contains("$ cd ..") {
                sizes.push(temp);
                return temp;
            } else if l.contains("$ cd") {
                temp += evaluate(lines, sizes);
            } else if !l.contains("$ ls") && !l.contains("dir ") {
                let (left, _) = l.split_once(' ').unwrap();
                temp += left.parse::<i32>().unwrap();
            }
        }
        return temp;
    }
    evaluate(_data.borrow_mut(), &mut sizes);
    dbg!(sizes.iter().filter(|x| x > &&(30000000 - 21618835)).min());
}
