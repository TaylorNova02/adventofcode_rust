pub fn run(input: Vec<String>) {
    let part1 = solve(input.clone(), 3);
    println!("part 1: {}", part1);
    assert_eq!(part1, 10723906903);

    let part2 = solve(input.clone(), 4);
    println!("part 2: {}", part2);
    assert_eq!(part2, 74850409);
}
fn solve(input: Vec<String>, number: i64) -> i64 {
    let target: i64 = input
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .sum::<i64>()
        / number;

    let mut possible_combinations: Vec<Vec<i64>> = Vec::new();
    let mut min_length = usize::MAX;

    ///
    /// Recursive function to find all possible combinations of numbers that sum to the target
    fn find_combinations(
        numbers: &[i64],
        target: i64,
        current_combination: &mut Vec<i64>,
        all_combinations: &mut Vec<Vec<i64>>,
        min_length: &mut usize,
    ) {
        let current_sum: i64 = current_combination.iter().sum();
        if current_sum == target {
            if current_combination.len() < *min_length {
                *min_length = current_combination.len();
                all_combinations.clear();
            }
            if current_combination.len() == *min_length {
                all_combinations.push(current_combination.clone());
            }
            return;
        }
        if current_sum > target {
            return;
        }
        for (i, &number) in numbers.iter().enumerate() {
            if current_sum + number > target {
                continue;
            }
            current_combination.push(number);
            find_combinations(
                &numbers[i + 1..],
                target,
                current_combination,
                all_combinations,
                min_length,
            );
            current_combination.pop();
        }
    }

    find_combinations(
        &input
            .iter()
            .map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
        target,
        &mut Vec::new(),
        &mut possible_combinations,
        &mut min_length,
    );
    let mut sorted_combinations: Vec<(Vec<i64>, i64)> = possible_combinations
        .iter()
        .map(|v| (v.clone(), v.iter().product::<i64>()))
        .collect();
    sorted_combinations.sort_by_key(|k| k.1);
    sorted_combinations[0].1
}
