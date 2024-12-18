use fancy_regex::*;
use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 236);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 51);
}

fn solve_part1(input: &[String]) -> u64 {
    let mut sum: u64 = 0;
    'outer: for line in input.iter() {
        let s: Vec<char> = line.chars().collect();
        // check that we have 3 vowels
        if count_vowels(&s) < 3 {
            continue;
        }

        // check that we have at least one letter which appears twice in a row
        let mut ok = false;
        for i in 0..(s.len() - 1) {
            if s[i] == s[i + 1] {
                ok = true;
                break;
            }
        }
        if !ok {
            continue;
        }

        // look for some "bad" substrings
        let bad_strings = ["ab", "cd", "pq", "xy"];
        for bad in bad_strings.iter() {
            if line.contains(bad) {
                continue 'outer;
            }
        }

        sum += 1;
    }
    sum
}

fn count_vowels(input: &[char]) -> u64 {
    let mut vowels = HashSet::new();
    vowels.insert('a');
    vowels.insert('e');
    vowels.insert('i');
    vowels.insert('o');
    vowels.insert('u');
    let mut count = 0;
    for c in input.iter() {
        if vowels.contains(c) {
            count += 1;
        }
    }
    count
}

fn solve_part2(input: &[String]) -> u64 {
    let rule1 = Regex::new(r"(..).*\1").unwrap();
    let rule2 = Regex::new(r"(.).\1").unwrap();

    let mut sum: u64 = 0;
    for line in input.iter() {
        // part2 is much easier to solve with regular expressions (once you find the right crate!)
        if !rule1.is_match(line).unwrap() {
            continue;
        }
        if !rule2.is_match(line).unwrap() {
            continue;
        }
        sum += 1;
    }
    sum
}
