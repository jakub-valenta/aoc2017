mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;

use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Invalid arguments!");
        return;
    }
    let puzzle = args[1].parse::<u32>().unwrap_or(100);
    let star = args[2].parse::<u32>().unwrap_or(100);
    if star > 2 || puzzle > 25 {
        println!("Invalid arguments!");
        return;
    }
    match puzzle {
        1 => {
            let captcha = if star == 1 {
                aoc1::sum_digits(&args[3])
            } else {
                aoc1::sum_digits_half(&args[3])
            };
            match captcha {
                Some(x) => println!("Captcha solved {}!", x),
                None => println!("Invalid captcha input!"),
            };
        }
        2 => {
            let checksum = if star == 1 {
                aoc2::checksum(&merge_args(&args, 3, "\n"))
            } else {
                aoc2::checksum_div(&merge_args(&args, 3, "\n"))
            };
            match checksum {
                Ok(x) => println!("Checksum calculated {}!", x),
                Err(e) => println!("Invalid checksum input {}!", e.description()),
            }
        }
        3 => {
            let cell = args[3].parse::<u32>();
            match cell {
                Ok(cell) => {
                    let steps = aoc3::distance(cell);
                    match steps {
                        Some(x) => println!("Distance calculated {}!", x),
                        None => println!("Invalid cell number {}!", cell),
                    }
                }
                Err(err) => println!("Invalid input {}!", err),
            }
        }
        4 => {
            let valid = if star == 1 {
                aoc4::verify_pass_phrase(&merge_args(&args, 3, " "))
            } else {
                aoc4::verify_pass_phrase_anagram(&merge_args(&args, 3, " "))
            };
            if valid {
                println!("Correct passphrase!");
            } else {
                println!("Incorrect passphrase!");
            }
        }
        _ => println!("Unknown puzzle!"),
    }
}

fn merge_args(args: &Vec<String>, first: usize, separator: &str) -> String {
    args.iter().skip(first).fold(String::new(), |acc, item| {
        acc + item + separator
    })
}
