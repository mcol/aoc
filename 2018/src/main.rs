use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: cargo run <day to solve>");
        return;
    }
    match args[1].parse::<u8>().expect("positive integer required") {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4 => day04::day04(),
        5 => day05::day05(),
        6 => day06::day06(),
        7 => day07::day07(),
        8 => day08::day08(),
        9 => day09::day09(),
        10 => day10::day10(),
        11 => day11::day11(),
        day @ 1..=25 => println!("Day {day} not solved yet"),
        _ => println!("Day must be between 1 and 25"),
    }
}
