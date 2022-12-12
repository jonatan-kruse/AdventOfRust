
pub fn star1(){
    println!("{}", include_str!("in_2.txt").lines()
        .map(|x| x.split(',')
        .map(|x| x.split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>()).filter(|pair| 
        (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1]) || 
        (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1])).count());
}