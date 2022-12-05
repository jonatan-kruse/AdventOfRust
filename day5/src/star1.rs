pub fn star1() {
    let (data, moves) = include_str!("in_2.txt").split_once('&').unwrap();
    let mut _data: Vec<&str> = data.lines().collect();
    _data.pop();

    let len = &_data[0].len();
    let mut stacks: Vec<Vec<char>> = vec![];
    for i in 2..*len {
        if (i - 2) % 4 == 0 {
            stacks.push(vec![]);
            _data.clone().into_iter().for_each(|j|
                if j.chars().nth(i - 1).unwrap() != ' ' {
                    stacks[(i - 2) / 4].push(j.chars().nth(i - 1).unwrap());
                }
            )
        }
    }
    let mut stacks: Vec<Vec<char>> = stacks
        .into_iter()
        .map(|stack| stack.into_iter().rev().collect::<Vec<char>>())
        .collect();

    let moves: Vec<Vec<usize>> = moves
        .lines()
        .into_iter()
        .map(|line| {
            line.replace("move", "")
                .replace("from", "")
                .replace("to", "")
                .split(' ')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect();

    for mov in moves {
        if !mov.is_empty() {
            for _i in 0..mov[0] {
                if let Some(char) = stacks[mov[1] - 1].pop() {
                    stacks[mov[2] - 1].push(char);
                }
            }
        }
    }
    dbg!(stacks);
}
