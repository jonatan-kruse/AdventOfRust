pub fn star2() {
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
                .map(|x| x - 1)
                .collect::<Vec<usize>>()
        })
        .collect();

    for mov in moves {
        let pax = stacks.clone();
        if !mov.is_empty() {
            let n = pax[mov[1]].len() - mov[0] - 1;
            let (vec1, vec2) = pax[mov[1]].split_at(n);
            stacks[mov[1]] = vec1.to_vec();
            stacks[mov[2]].append(&mut vec2.to_vec());
        }
    }
    dbg!(stacks);
}
