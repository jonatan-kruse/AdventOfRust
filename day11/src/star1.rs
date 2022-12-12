pub fn star1() {
    let _data = include_str!("in_2.txt")
        .split("\r\n\r\n")
        .collect::<Vec<&str>>();
    #[derive(Clone, Copy, Debug)]
    enum Opperation {
        Pow,
        Mul,
        Add,
    }
    #[derive(Clone, Debug)]
    struct Monkey {
        div_by: usize,
        trow_if_true: usize,
        trow_if_false: usize,
        opperation: Opperation,
        opperation_value: usize,
        items: Vec<usize>,
    }
    let mut monkeys: Vec<Monkey> = _data
        .iter()
        .map(|lines| {
            let lines = lines.split("\r\n").collect::<Vec<&str>>();
            let mut iter_lines = lines.iter();
            let items: Vec<usize> = iter_lines
                .next()
                .unwrap()
                .split(' ')
                .into_iter()
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            let (op, op_value) = iter_lines.next().unwrap().split_once(' ').unwrap();
            let opperation;
            let opperation_value;
            if op_value == "old" {
                opperation = Opperation::Pow;
                opperation_value = 2;
            } else if op == "+" {
                opperation = Opperation::Add;
                opperation_value = op_value.parse().unwrap();
            } else {
                opperation = Opperation::Mul;
                opperation_value = op_value.parse().unwrap();
            }
            let div_by: usize = iter_lines.next().unwrap().parse().unwrap();
            let trow_if_true: usize = iter_lines.next().unwrap().parse().unwrap();
            let trow_if_false: usize = iter_lines.next().unwrap().parse().unwrap();
            Monkey {
                div_by,
                trow_if_true,
                trow_if_false,
                opperation,
                opperation_value,
                items,
            }
        })
        .collect();
    let lcm: usize = monkeys.iter().map(|m| m.div_by).product();
    let mut inspects = vec![];
    monkeys.iter().for_each(|_| inspects.push(0));
    (0..10000).into_iter().for_each(|_| {
        (0..monkeys.len()).into_iter().for_each(|index| {
            let monkey = monkeys[index].clone();
            inspects[index] += monkey.items.len();
            monkey.items.iter().for_each(|i| {
                let mut item = *i;
                match &monkey.opperation {
                    Opperation::Add => item += &monkey.opperation_value,
                    Opperation::Pow => item = item * item,
                    Opperation::Mul => item *= &monkey.opperation_value,
                }
                item %= lcm;
                if item % monkey.div_by == 0 {
                    monkeys[monkey.trow_if_true].items.push(item);
                } else {
                    monkeys[monkey.trow_if_false].items.push(item);
                }
                monkeys[index].items = vec![];
            })
        })
    });
    dbg!(&inspects);
    inspects.sort();
    inspects.reverse();
    dbg!(inspects.iter().take(2).product::<usize>());
}
