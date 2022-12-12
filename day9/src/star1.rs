pub fn star1() {
    let _data = include_str!("in_1.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .collect();
    hej
}
