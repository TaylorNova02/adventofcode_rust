use divisors;

pub fn run(input: u64) {
    let part1 = solve1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 786240);

    let part2 = solve2(input);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 1006);
}
fn solve1(input: u64) -> u64 {
    let mut house_nbr: u64 = 1;
    loop {
        let mut n =
            divisors::get_divisors(house_nbr).iter().sum::<u64>() * 10 + (house_nbr + 1) * 10;
        if house_nbr == 1 {
            n = 10;
        }
        if n >= input {
            break;
        }
        house_nbr += 1;
    }

    house_nbr
}
fn solve2(input: u64) -> u64 {
    let mut house_nbr: u64 = 1;
    loop {
        let mut n = 0;
        let max_visits = 50;
        let mut divisors = divisors::get_divisors(house_nbr);
        divisors.push(house_nbr);
        divisors.push(1);
        // println!("{}, {:?}", house_nbr, divisors);
        for divisor in divisors {
            if divisor * max_visits >= house_nbr {
                n += divisor * 11;
            }
        }
        if n >= input {
            break;
        }
        house_nbr += 1;
    }

    house_nbr
}
