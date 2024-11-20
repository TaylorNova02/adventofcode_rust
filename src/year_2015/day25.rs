pub fn run(input: String) {
    let part1 = solve(input.clone());
    println!("part 1: {}", part1);
    // assert_eq!(part1, 0);

    println!("part 2: Doesn't exist ^^",);
}
fn solve(input: String) -> i64 {
    let mut x: i32 = 1;
    let mut y: i32 = 1;
    let re = regex::Regex::new(r"row (\d+), column (\d+)").unwrap();
    let captures = re.captures(&input).unwrap();

    let row: i32 = captures[1].parse().unwrap();
    let col: i32 = captures[2].parse().unwrap();
    let mut code = 20151125;
    while x != col || y != row {
        if y == 1 {
            y = x + 1;
            x = 1;
        } else {
            x += 1;
            y -= 1;
        }
        code = (code * 252533) % 33554393;
    }
    code
}
