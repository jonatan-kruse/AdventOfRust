pub fn star1() {
    let _data = include_str!("in_2.txt").lines();
    let mut ans = 0;
    let mut x = 1;
    let mut clock_cycle = 0;
    _data.into_iter().for_each(|l| {
        if l.contains("noop") {
            clock_cycle += 1;
            if (clock_cycle + 20) % 40 == 0 {
                ans += clock_cycle * x;
            }
        } else {
            let (_, num) = l.split_once(' ').unwrap();
            clock_cycle += 1;
            if (clock_cycle + 20) % 40 == 0 {
                ans += clock_cycle * x;
            }
            clock_cycle += 1;
            if (clock_cycle + 20) % 40 == 0 {
                ans += clock_cycle * x;
            }
            x += num.parse::<i32>().unwrap();
        }
    });
    dbg!(ans);
}
