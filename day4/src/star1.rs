
pub fn star1(){
    let data:Vec<&str> = include_str!("in_2.txt").lines().collect();
    let data2: Vec<Vec<Vec<i32>>> = data.into_iter()
        .map(|x| x.split(",")
        .map(|x| x.split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()).collect()).collect();
    
    let ans: Vec<Vec<Vec<i32>>> = data2.into_iter().filter(|pair| 
        (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1]) || 
        (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1])).collect();

    println!("{}", ans.len());
}