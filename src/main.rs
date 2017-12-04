mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;

use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid arguments!");
        return;
    }
    let puzzle = args[1].parse::<u32>();
    match puzzle {
        Ok(1) => {
            let captcha = aoc1::sum_digits(&args[2]);
            match captcha {
                Some(x) => println!("Captcha solved {}!", x),
                None => println!("Invalid captcha input!"),
            };
        }
        Ok(2) => {
            let checksum = aoc2::checksum(&merge_args(&args, 2, "\n"));
            match checksum {
                Ok(x) => println!("Checksum calculated {}!", x),
                Err(e) => println!("Invalid checksum input {}!", e.description()),
            }
        }
        Ok(3) => {
            let cell = args[2].parse::<u32>();
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
        Ok(4) => {
            if aoc4::verify_pass_phrase(&merge_args(&args, 2, " ")) {
                println!("Passphrase correct!");
            } else {
                println!("Passphrase incorrect!");
            }
        }
        Ok(_) => println!("Unknown puzzle!"),
        Err(err) => println!("Invalid input {}!", err),
    }
}

fn merge_args(args: &Vec<String>, first: usize, separator: &str) -> String {
    args.iter().skip(first).fold(String::new(), |acc, item| {
        acc + item + separator
    })
}
