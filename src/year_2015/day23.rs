pub fn run(input: Vec<String>) {
    let part1 = solve(input.clone(), 0, 0);
    println!("part 1: {}", part1);
    // assert_eq!(part1, 0);

    let part2 = solve(input.clone(), 1, 0);
    println!("part 2: {}", part2);
    // assert_eq!(part2, 0);
}
fn solve(input: Vec<String>, reg_a: i64, reg_b: i64) -> i64 {
    let mut current_pos: i64 = 0;
    let mut register_a = reg_a;
    let mut register_b = reg_b;
    while current_pos < input.len() as i64 {
        let instruction: Vec<&str> = input[current_pos as usize].split_whitespace().collect();
        let command = instruction[0];
        let register = instruction[1].chars().next().unwrap();
        let value = instruction.get(2).unwrap_or(&&"0").parse::<i64>().unwrap();
        if command == "jmp" {
            let offset = instruction[1].parse::<i64>().unwrap();
            current_pos += offset;
            continue;
        }
        match command {
            "hlf" => {
                if register == 'a' {
                    register_a /= 2;
                } else {
                    register_b /= 2;
                }
            }
            "tpl" => {
                if register == 'a' {
                    register_a *= 3;
                } else {
                    register_b *= 3;
                }
            }
            "inc" => {
                if register == 'a' {
                    register_a += 1;
                } else {
                    register_b += 1;
                }
            }
            "jie" => {
                let offset = value;
                if register == 'a' {
                    if register_a % 2 == 0 {
                        current_pos += offset;
                        continue;
                    }
                } else if register_b % 2 == 0 {
                    current_pos += offset;
                    continue;
                }
            }
            "jio" => {
                let offset = value;
                if register == 'a' {
                    if register_a == 1 {
                        current_pos += offset;
                        continue;
                    }
                } else if register_b == 1 {
                    current_pos += offset;
                    continue;
                }
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
        current_pos += 1;
    }

    register_b
}
