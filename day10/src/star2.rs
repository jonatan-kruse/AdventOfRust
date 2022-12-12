pub fn star2() {
    let _data = include_str!("in_2.txt").lines();
    let mut drawing: Vec<Vec<char>> = vec![vec![]];
    let mut x = 1;
    let mut row = 0;
    let mut clock_cycle = 0;
    _data.into_iter().for_each(|l| {
        if l.contains("noop") {
            clock_cycle += 1; 
            if (clock_cycle - 1) % 40 == 0 {
                dbg!(x);
                row += 1;
                drawing.push(vec![]);
            }
            if x - 1 > clock_cycle % 40 - 1 || x + 1 < clock_cycle % 40 - 1 {
                drawing[row].push('.');
            } else {
                drawing[row].push('#');
            }
           
        } else {
            let (_, num) = l.split_once(' ').unwrap();
            clock_cycle += 1;
            if (clock_cycle - 1) % 40 == 0 {
                dbg!(x);
                row += 1;
                drawing.push(vec![]);
            }
            if x - 1 > clock_cycle % 40 - 1 || x + 1 < clock_cycle % 40 - 1 {
                drawing[row].push('.');
            } else {
                drawing[row].push('#');
            }
            clock_cycle += 1;
            if (clock_cycle - 1)% 40 == 0 {
                dbg!(x);
                row += 1;
                drawing.push(vec![]);
            }
            if x - 1 > clock_cycle % 40 - 1 || x + 1 < clock_cycle % 40 - 1 {
                drawing[row].push('.');
            } else {
                drawing[row].push('#');
            }
            x += num.parse::<i32>().unwrap();
        }
    });
    drawing.iter().for_each(|row|{
        println!();
        row.iter().for_each(|c| print!("{}", c));
    });
}
