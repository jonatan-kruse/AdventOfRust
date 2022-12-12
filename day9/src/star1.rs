use std::collections::HashSet;

pub fn star1() {
    let _data: Vec<(char, i32)> = include_str!("in_2.txt")
        .lines()
        .map(|l| {
            let (dir, amount) = l.split_once(' ').unwrap();
            (dir.parse().unwrap(), amount.parse().unwrap())
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    _data.iter().for_each(|(dir, amount)| {
        let direction = match dir {
            'D' => (0, -1),
            'U' => (0, 1),
            'R' => (1, 0),
            'L' => (-1, 0),
            _ => (0, 0),
        };
        (0..*amount).into_iter().for_each(|_| {
            head = (head.0 + direction.0, head.1 + direction.1);
            let dif = (head.0 - tail.0, head.1 - tail.1);
            match dif {
                (2, 0) => tail = (tail.0 + 1, tail.1),
                (-2, 0) => tail = (tail.0 - 1, tail.1),
                (0, 2) => tail = (tail.0, tail.1 + 1),
                (0, -2) => tail = (tail.0, tail.1 - 1),

                (2, 1) => tail = (tail.0 + 1, tail.1 + 1),
                (-2, 1) => tail = (tail.0 - 1, tail.1 + 1),
                (1, 2) => tail = (tail.0 + 1, tail.1 + 1),
                (1, -2) => tail = (tail.0 + 1, tail.1 - 1),

                (2, -1) => tail = (tail.0 + 1, tail.1 - 1),
                (-2, -1) => tail = (tail.0 - 1, tail.1 - 1),
                (-1, 2) => tail = (tail.0 - 1, tail.1 + 1),
                (-1, -2) => tail = (tail.0 - 1, tail.1 - 1),
                _ => {}
            }
            visited.insert(tail);
        });
    });
    dbg!(visited.len());
}
