use std::env;

mod common;
mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;
mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;
mod year_2023;

extern crate crypto;
extern crate fancy_regex;
extern crate regex;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

use common::inputs;

fn main() {
    // Convert argv to a Vec<String>, so we can access elements using [] notation.
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 {
        panic!("Usage: cargo run <year> <day>");
    }

    // In general, Rust can infer types for local variables. Here, we need to tell the type
    // inference which type we want to parse the String into because a String can be parsed into
    // many different types.
    let year: u32 = argv[1].parse().expect("Year must be a number");
    let day: u8 = argv[2].parse().expect("Day must be a number");
    println!("Running year {}, day {}", year, day);

    match (year, day) {
        // 2015
        (2015, 1) => year_2015::day01::run(&inputs::read_first_line(year, day)),
        (2015, 2) => year_2015::day02::run(inputs::read(year, day)),
        (2015, 3) => year_2015::day03::run(&inputs::read_first_line(year, day)),
        (2015, 4) => year_2015::day04::run("ckczppom"),
        (2015, 5) => year_2015::day05::run(inputs::read(year, day)),
        (2015, 6) => year_2015::day06::run(inputs::read(year, day)),
        (2015, 7) => year_2015::day07::run(inputs::read(year, day)),
        (2015, 8) => year_2015::day08::run(inputs::read(year, day)),
        (2015, 9) => year_2015::day09::run(inputs::read(year, day)),
        (2015, 10) => year_2015::day10::run("3113322113"),
        (2015, 11) => year_2015::day11::run("vzbxkghb"),
        (2015, 12) => year_2015::day12::run(&inputs::read_first_line(year, day)),
        (2015, 13) => year_2015::day13::run(inputs::read(year, day)),
        (2015, 14) => year_2015::day14::run(inputs::read(year, day)),
        (2015, 15) => year_2015::day15::run(inputs::read(year, day)),
        (2015, 16) => year_2015::day16::run(inputs::read(year, day)),
        (2015, 17) => year_2015::day17::run(inputs::read(year, day)),
        (2015, 18) => year_2015::day18::run(inputs::read(year, day)),
        // (2015, 19) => year_2015::day19::run(inputs::read(year, day)),
        // (2015, 20) => year_2015::day20::run(inputs::read(year, day)),
        // (2015, 21) => year_2015::day21::run(inputs::read(year, day)),
        // (2015, 22) => year_2015::day22::run(inputs::read(year, day)),
        // (2015, 23) => year_2015::day23::run(inputs::read(year, day)),
        // (2015, 24) => year_2015::day24::run(inputs::read(year, day)),
        // (2015, 25) => year_2015::day25::run(inputs::read(year, day)),
        (2015, _) => println!("work in progress..."),

        // 2016
        (2016, 1) => year_2016::day01::run(&inputs::grab_local_input(year, day)),
        (2016, 2) => year_2016::day02::run(&inputs::grab_local_input(year, day)),
        (2016, 3) => year_2016::day03::run(&inputs::grab_local_input(year, day)),
        (2016, 4) => year_2016::day04::run(&inputs::grab_local_input(year, day)),
        (2016, 5) => year_2016::day05::run("ojvtpuvg"),
        (2016, 6) => year_2016::day06::solve(&inputs::grab_local_input(year, day)),
        (2016, 7) => year_2016::day07::solve(&inputs::grab_local_input(year, day)),
        (2016, 8) => year_2016::day08::solve(&inputs::grab_local_input(year, day)),
        (2016, 9) => year_2016::day09::solve(&inputs::grab_local_input(year, day)),
        (2016, 10) => year_2016::day10::solve(&inputs::grab_local_input(year, day)),
        (2016, 11) => year_2016::day10::solve(&inputs::grab_local_input(year, day)),
        // (2016, 12) => year_2016::day12::solve(&inputs::grab_local_input(year, day)),
        // (2016, 13) => year_2016::day13::solve(&inputs::grab_local_input(year, day)),
        (2016, 14) => year_2016::day14::solve("qzyelonm"),
        (2016, 15) => year_2016::day15::solve(&inputs::grab_local_input(year, day)),
        (2016, 16) => year_2016::day16::solve("01111001100111011"),
        // (2016, 17) => year_2016::day17::solve(&inputs::grab_local_input(year, day)),
        (2016, 18) => year_2016::day18::solve(&inputs::grab_local_input(year, day)),
        (2016, 19) => year_2016::day19::solve(3005290),
        (2016, 20) => year_2016::day20::solve(&inputs::grab_local_input(year, day)),
        (2016, 21) => year_2016::day21::solve(&inputs::grab_local_input(year, day), "abcdefgh", "fbgdceah"),
        // (2016, 22) => year_2016::day22::solve(&inputs::grab_local_input(year, day)),
        //(2016, 23) => year_2016::day23::solve(&inputs::grab_local_input(year, day)),
        // (2016, 24) => year_2016::day24::solve(&inputs::grab_local_input(year, day)),
        // (2016, 25) => year_2016::day25::solve(&inputs::grab_local_input(year, day)),
        (2016, _) => println!("work in progress..."),

        // 2017
        (2017, 1) => year_2017::day01::run(&inputs::read_first_line(year, day)),
        (2017, 2) => year_2017::day02::run(inputs::read(year, day)),
        (2017, 3) => year_2017::day03::run(368078),
        (2017, 4) => year_2017::day04::run(inputs::read(year, day)),
        (2017, 5) => year_2017::day05::run(inputs::read(year, day)),
        (2017, 6) => year_2017::day06::run(&inputs::read_first_line(year, day)),
        (2017, 7) => year_2017::day07::run(inputs::read(year, day)),
        (2017, 8) => year_2017::day08::run(inputs::read(year, day)),
        (2017, 9) => year_2017::day09::run(&inputs::read_first_line(year, day)),
        (2017, 10) => year_2017::day10::run(&inputs::read_first_line(year, day)),
        (2017, 11) => year_2017::day11::run(&inputs::read_first_line(year, day)),
        (2017, 12) => year_2017::day12::run(inputs::read(year, day)),
        (2017, 13) => year_2017::day13::run(inputs::read(year, day)),
        (2017, 14) => year_2017::day14::run("wenycdww"),
        (2017, 15) => year_2017::day15::run(591, 393),
        (2017, 16) => year_2017::day16::run(&inputs::read_first_line(year, day)),
        (2017, 17) => year_2017::day17::run(354),
        (2017, 18) => year_2017::day18::run(inputs::read(year, day)),
        (2017, 19) => year_2017::day19::run(inputs::read(year, day)),
        (2017, 20) => year_2017::day20::run(inputs::read(year, day)),
        (2017, 21) => year_2017::day21::run(inputs::read(year, day)),
        (2017, 22) => year_2017::day22::run(inputs::read(year, day)),
        (2017, 23) => year_2017::day23::run(inputs::read(year, day)),
        (2017, 24) => year_2017::day24::run(inputs::read(year, day)),
        (2017, 25) => year_2017::day25::run(),

        //2018
        // (2018, 1) => year_2018::day01::run(&inputs::read_first_line(year, day)),
        // (2018, 2) => year_2018::day02::run(inputs::read(year, day)),
        // (2018, 3) => year_2018::day03::run(368078),
        // (2018, 4) => year_2018::day04::run(inputs::read(year, day)),
        // (2018, 5) => year_2018::day05::run(inputs::read(year, day)),
        // (2018, 6) => year_2018::day06::run(&inputs::read_first_line(year, day)),
        // (2018, 7) => year_2018::day07::run(inputs::read(year, day)),
        // (2018, 8) => year_2018::day08::run(inputs::read(year, day)),
        // (2018, 9) => year_2018::day09::run(&inputs::read_first_line(year, day)),
        // (2018, 10) => year_2018::day10::run(&inputs::read_first_line(year, day)),
        // (2018, 11) => year_2018::day11::run(&inputs::read_first_line(year, day)),
        // (2018, 12) => year_2018::day12::run(inputs::read(year, day)),
        // (2018, 13) => year_2018::day13::run(inputs::read(year, day)),
        // (2018, 14) => year_2018::day14::run("wenycdww"),
        // (2018, 15) => year_2018::day15::run(591, 393),
        // (2018, 16) => year_2018::day16::run(&inputs::read_first_line(year, day)),
        // (2018, 17) => year_2018::day17::run(354),
        // (2018, 18) => year_2018::day18::run(inputs::read(year, day)),
        // (2018, 19) => year_2018::day19::run(inputs::read(year, day)),
        // (2018, 20) => year_2018::day20::run(inputs::read(year, day)),
        // (2018, 21) => year_2018::day21::run(inputs::read(year, day)),
        // (2018, 22) => year_2018::day22::run(inputs::read(year, day)),
        // (2018, 23) => year_2018::day23::run(inputs::read(year, day)),
        // (2018, 24) => year_2018::day24::run(inputs::read(year, day)),
        // (2018, 25) => year_2018::day25::run(),

        //2019
        // (2019, 1) => year_2019::day01::run(&inputs::read_first_line(year, day)),
        // (2019, 2) => year_2019::day02::run(inputs::read(year, day)),
        // (2019, 3) => year_2019::day03::run(368078),
        // (2019, 4) => year_2019::day04::run(inputs::read(year, day)),
        // (2019, 5) => year_2019::day05::run(inputs::read(year, day)),
        // (2019, 6) => year_2019::day06::run(&inputs::read_first_line(year, day)),
        // (2019, 7) => year_2019::day07::run(inputs::read(year, day)),
        // (2019, 8) => year_2019::day08::run(inputs::read(year, day)),
        // (2019, 9) => year_2019::day09::run(&inputs::read_first_line(year, day)),
        // (2019, 10) => year_2019::day10::run(&inputs::read_first_line(year, day)),
        // (2019, 11) => year_2019::day11::run(&inputs::read_first_line(year, day)),
        // (2019, 12) => year_2019::day12::run(inputs::read(year, day)),
        // (2019, 13) => year_2019::day13::run(inputs::read(year, day)),
        // (2019, 14) => year_2019::day14::run("wenycdww"),
        // (2019, 15) => year_2019::day15::run(591, 393),
        // (2019, 16) => year_2019::day16::run(&inputs::read_first_line(year, day)),
        // (2019, 17) => year_2019::day17::run(354),
        // (2019, 18) => year_2019::day18::run(inputs::read(year, day)),
        // (2019, 19) => year_2019::day19::run(inputs::read(year, day)),
        // (2019, 20) => year_2019::day20::run(inputs::read(year, day)),
        // (2019, 21) => year_2019::day21::run(inputs::read(year, day)),
        // (2019, 22) => year_2019::day22::run(inputs::read(year, day)),
        // (2019, 23) => year_2019::day23::run(inputs::read(year, day)),
        // (2019, 24) => year_2019::day24::run(inputs::read(year, day)),
        // (2019, 25) => year_2019::day25::run(),

        //2020
        // (2020, 1) => year_2020::day01::run(&inputs::read_first_line(year, day)),
        // (2020, 2) => year_2020::day02::run(inputs::read(year, day)),
        // (2020, 3) => year_2020::day03::run(368078),
        // (2020, 4) => year_2020::day04::run(inputs::read(year, day)),
        // (2020, 5) => year_2020::day05::run(inputs::read(year, day)),
        // (2020, 6) => year_2020::day06::run(&inputs::read_first_line(year, day)),
        // (2020, 7) => year_2020::day07::run(inputs::read(year, day)),
        // (2020, 8) => year_2020::day08::run(inputs::read(year, day)),
        // (2020, 9) => year_2020::day09::run(&inputs::read_first_line(year, day)),
        // (2020, 10) => year_2020::day10::run(&inputs::read_first_line(year, day)),
        // (2020, 11) => year_2020::day11::run(&inputs::read_first_line(year, day)),
        // (2020, 12) => year_2020::day12::run(inputs::read(year, day)),
        // (2020, 13) => year_2020::day13::run(inputs::read(year, day)),
        // (2020, 14) => year_2020::day14::run("wenycdww"),
        // (2020, 15) => year_2020::day15::run(591, 393),
        // (2020, 16) => year_2020::day16::run(&inputs::read_first_line(year, day)),
        // (2020, 17) => year_2020::day17::run(354),
        // (2020, 18) => year_2020::day18::run(inputs::read(year, day)),
        // (2020, 19) => year_2020::day19::run(inputs::read(year, day)),
        // (2020, 20) => year_2020::day20::run(inputs::read(year, day)),
        // (2020, 21) => year_2020::day21::run(inputs::read(year, day)),
        // (2020, 22) => year_2020::day22::run(inputs::read(year, day)),
        // (2020, 23) => year_2020::day23::run(inputs::read(year, day)),
        // (2020, 24) => year_2020::day24::run(inputs::read(year, day)),
        // (2020, 25) => year_2020::day25::run(),

        //2021
        // (2021, 1) => year_2021::day01::run(&inputs::read_first_line(year, day)),
        // (2021, 2) => year_2021::day02::run(inputs::read(year, day)),
        // (2021, 3) => year_2021::day03::run(368078),
        // (2021, 4) => year_2021::day04::run(inputs::read(year, day)),
        // (2021, 5) => year_2021::day05::run(inputs::read(year, day)),
        // (2021, 6) => year_2021::day06::run(&inputs::read_first_line(year, day)),
        // (2021, 7) => year_2021::day07::run(inputs::read(year, day)),
        // (2021, 8) => year_2021::day08::run(inputs::read(year, day)),
        // (2021, 9) => year_2021::day09::run(&inputs::read_first_line(year, day)),
        // (2021, 10) => year_2021::day10::run(&inputs::read_first_line(year, day)),
        // (2021, 11) => year_2021::day11::run(&inputs::read_first_line(year, day)),
        // (2021, 12) => year_2021::day12::run(inputs::read(year, day)),
        // (2021, 13) => year_2021::day13::run(inputs::read(year, day)),
        // (2021, 14) => year_2021::day14::run("wenycdww"),
        // (2021, 15) => year_2021::day15::run(591, 393),
        // (2021, 16) => year_2021::day16::run(&inputs::read_first_line(year, day)),
        // (2021, 17) => year_2021::day17::run(354),
        // (2021, 18) => year_2021::day18::run(inputs::read(year, day)),
        // (2021, 19) => year_2021::day19::run(inputs::read(year, day)),
        // (2021, 20) => year_2021::day20::run(inputs::read(year, day)),
        // (2021, 21) => year_2021::day21::run(inputs::read(year, day)),
        // (2021, 22) => year_2021::day22::run(inputs::read(year, day)),
        // (2021, 23) => year_2021::day23::run(inputs::read(year, day)),
        // (2021, 24) => year_2021::day24::run(inputs::read(year, day)),
        // (2021, 25) => year_2021::day25::run(),

        //2022
        // (2022, 1) => year_2022::day01::run(&inputs::read_first_line(year, day)),
        // (2022, 2) => year_2022::day02::run(inputs::read(year, day)),
        // (2022, 3) => year_2022::day03::run(368078),
        // (2022, 4) => year_2022::day04::run(inputs::read(year, day)),
        // (2022, 5) => year_2022::day05::run(inputs::read(year, day)),
        // (2022, 6) => year_2022::day06::run(&inputs::read_first_line(year, day)),
        // (2022, 7) => year_2022::day07::run(inputs::read(year, day)),
        // (2022, 8) => year_2022::day08::run(inputs::read(year, day)),
        // (2022, 9) => year_2022::day09::run(&inputs::read_first_line(year, day)),
        // (2022, 10) => year_2022::day10::run(&inputs::read_first_line(year, day)),
        // (2022, 11) => year_2022::day11::run(&inputs::read_first_line(year, day)),
        // (2022, 12) => year_2022::day12::run(inputs::read(year, day)),
        // (2022, 13) => year_2022::day13::run(inputs::read(year, day)),
        // (2022, 14) => year_2022::day14::run("wenycdww"),
        // (2022, 15) => year_2022::day15::run(591, 393),
        // (2022, 16) => year_2022::day16::run(&inputs::read_first_line(year, day)),
        // (2022, 17) => year_2022::day17::run(354),
        // (2022, 18) => year_2022::day18::run(inputs::read(year, day)),
        // (2022, 19) => year_2022::day19::run(inputs::read(year, day)),
        // (2022, 20) => year_2022::day20::run(inputs::read(year, day)),
        // (2022, 21) => year_2022::day21::run(inputs::read(year, day)),
        // (2022, 22) => year_2022::day22::run(inputs::read(year, day)),
        // (2022, 23) => year_2022::day23::run(inputs::read(year, day)),
        // (2022, 24) => year_2022::day24::run(inputs::read(year, day)),
        // (2022, 25) => year_2022::day25::run(),

        //2023
        // (2023, 1) => year_2023::day01::run(&inputs::read_first_line(year, day)),
        // (2023, 2) => year_2023::day02::run(inputs::read(year, day)),
        // (2023, 3) => year_2023::day03::run(368078),
        // (2023, 4) => year_2023::day04::run(inputs::read(year, day)),
        // (2023, 5) => year_2023::day05::run(inputs::read(year, day)),
        // (2023, 6) => year_2023::day06::run(&inputs::read_first_line(year, day)),
        // (2023, 7) => year_2023::day07::run(inputs::read(year, day)),
        // (2023, 8) => year_2023::day08::run(inputs::read(year, day)),
        // (2023, 9) => year_2023::day09::run(&inputs::read_first_line(year, day)),
        // (2023, 10) => year_2023::day10::run(&inputs::read_first_line(year, day)),
        // (2023, 11) => year_2023::day11::run(&inputs::read_first_line(year, day)),
        // (2023, 12) => year_2023::day12::run(inputs::read(year, day)),
        // (2023, 13) => year_2023::day13::run(inputs::read(year, day)),
        // (2023, 14) => year_2023::day14::run("wenycdww"),
        // (2023, 15) => year_2023::day15::run(591, 393),
        // (2023, 16) => year_2023::day16::run(&inputs::read_first_line(year, day)),
        // (2023, 17) => year_2023::day17::run(354),
        // (2023, 18) => year_2023::day18::run(inputs::read(year, day)),
        // (2023, 19) => year_2023::day19::run(inputs::read(year, day)),
        // (2023, 20) => year_2023::day20::run(inputs::read(year, day)),
        // (2023, 21) => year_2023::day21::run(inputs::read(year, day)),
        // (2023, 22) => year_2023::day22::run(inputs::read(year, day)),
        // (2023, 23) => year_2023::day23::run(inputs::read(year, day)),
        // (2023, 24) => year_2023::day24::run(inputs::read(year, day)),
        // (2023, 25) => year_2023::day25::run(),
        // Other
        (_, _) => panic!("Not implemented :("),
    }
}
