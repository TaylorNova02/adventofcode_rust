pub fn run(input: &[u8]) {
    let part1 = solve(input, false);
    println!("part 1: {}", part1);
    assert_eq!(part1, 74);

    let part2 = solve(input, true);
    println!("part 2: {}", part2);
    assert_eq!(part2, 1795);
}

fn solve(buf: &[u8], check_basement: bool) -> i64 {
    let mut floor: i64 = 0;
    // loop over every character
    for i in 0..buf.len() {
        if buf[i] == b'(' {
            // increment floor when we see a '('
            floor += 1;
        } else if buf[i] == b')' {
            // decrement floor when we see a ')'
            floor -= 1;
        } else {
            panic!("invalid input: {}", buf[i]);
        }
        if floor == -1 && check_basement {
            // return the 1-indexed position when we hit the basement
            return (i + 1) as i64;
        }
    }

    floor
}
