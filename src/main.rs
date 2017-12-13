mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;
mod aoc10;
mod aoc11;
mod utils;

use std::env;

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
                aoc2::checksum(&utils::merge_args(&args, 3, "\n"))
            } else {
                aoc2::checksum_div(&utils::merge_args(&args, 3, "\n"))
            };
            match checksum {
                Some(x) => println!("Checksum calculated {}!", x),
                None => println!("Invalid checksum input!"),
            }
        }
        3 => {
            let cell = args[3].parse::<u32>();
            match cell {
                Ok(cell) => {
                    let steps = if star == 1 {
                        aoc3::distance(cell)
                    } else {
                        aoc3::calculate_bigger_than(cell)
                    };
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
                aoc4::verify_pass_phrase(&utils::merge_args(&args, 3, " "))
            } else {
                aoc4::verify_pass_phrase_anagram(&utils::merge_args(&args, 3, " "))
            };
            if valid {
                println!("Correct passphrase!");
            } else {
                println!("Incorrect passphrase!");
            }
        }
        5 => {
            let steps = if star == 1 {
                aoc5::process_instructions(&utils::merge_args(&args, 3, " "))
            } else {
                aoc5::process_instructions_strange(&utils::merge_args(&args, 3, " "))
            };
            match steps {
                Some(x) => println!("Finished in {} steps!", x),
                None => println!("Invalid instructions input!"),
            }
        }
        6 => {
            match aoc6::detect_cycle(&utils::merge_args(&args, 3, " ")) {
                Some((steps, loop_length)) => {
                    println!(
                        "Finished in {} steps, infinite loop length is {}!",
                        steps,
                        loop_length
                    )
                }
                None => println!("Invalid instructions input!"),
            }
        }
        7 => {
            if star == 1 {
                match aoc7::find_bottom_program(&utils::merge_args(&args, 3, "\n")) {
                    Some(program) => println!("Bottom program is {}!", program),
                    None => println!("Invalid programs!"),
                }
            } else {
                match aoc7::correct_weight(&utils::merge_args(&args, 3, "\n")) {
                    Some(weight) => println!("Correct weight is {}!", weight),
                    None => println!("Invalid programs!"),
                }
            }
        }
        8 => {
            let max = if star == 1 {
                aoc8::find_max(&utils::merge_args(&args, 3, "\n"))
            } else {
                aoc8::find_max_total(&utils::merge_args(&args, 3, "\n"))
            };
            match max {
                Some(max) => println!("Max value in register is {}!", max),
                None => println!("Invalid programs!"),
            }
        }
        9 => {
            let result = aoc9::compute_score(&args[3]);
            match result {
                Some((score, garbage_count)) => {
                    if star == 1 {
                        println!("Score is {}!", score);
                    } else {
                        println!("Garbage count is {}!", garbage_count);
                    }
                }
                None => println!("Invalid input!"),
            }
        }
        10 => {
            if star == 1 {
                match aoc10::knot_tying_hash_round(&utils::merge_args(&args, 3, ",")) {
                    Some(hash) => println!("Hash is {}!", hash),
                    None => println!("Invalid input!"),
                }
            } else {
                println!(
                    "Hash is {}!",
                    aoc10::knot_tying_hash(&utils::merge_args(&args, 3, ""))
                );
            }
        }
        11 => {
            match aoc11::find_shortest_path(&args[3]) {
                Some(path) => println!("Shortest path is {} steps!", path),
                None => println!("Invalid input!"),
            }
        }
        _ => println!("Unknown puzzle!"),
    }
}
