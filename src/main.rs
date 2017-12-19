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
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;
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
            if star == 1 {
                match aoc11::find_shortest_path(&args[3]) {
                    Some(path) => println!("Shortest path is {} steps!", path),
                    None => println!("Invalid input!"),
                }
            } else {
                match aoc11::find_furthest_point(&args[3]) {
                    Some(path) => println!("Furthest point was {} steps away!", path),
                    None => println!("Invalid input!"),
                }
            }
        }
        12 => {
            if star == 1 {
                match aoc12::connected_programs(&utils::merge_args(&args, 3, "\n")) {
                    Some(group_size) => println!("Program count connected to '0': {}!", group_size),
                    None => println!("Invalid input!"),
                }
            } else {
                match aoc12::program_groups(&utils::merge_args(&args, 3, "\n")) {
                    Some(group_count) => println!("Independent program groups: {}!", group_count),
                    None => println!("Invalid input!"),
                }
            }
        }
        13 => {
            if star == 1 {
                match aoc13::compute_severity(&utils::merge_args(&args, 3, "\n")) {
                    Some(severity) => println!("Packet severity is {}!", severity),
                    None => println!("Invalid input!"),
                }
            } else {
                match aoc13::compute_delay(&utils::merge_args(&args, 3, "\n")) {
                    Some(severity) => println!("Packet must be delayed {} pico seconds!", severity),
                    None => println!("Invalid input!"),
                }
            }
        }
        14 => {
            if star == 1 {
                println!(
                    "There are {} used squares!",
                    aoc14::count_used_squares(&args[3])
                );
            } else {
                println!(
                    "There are {} memory regions!",
                    aoc14::count_regions(&args[3])
                );
            }
        }
        15 => {
            let matches = if star == 1 {
                aoc15::count_matches(&args[3], &args[4])
            } else {
                aoc15::count_filtered(&args[3], &args[4])
            };
            match matches {
                Some(matches) => println!("Judge found {} matches!", matches),
                None => println!("Invalid input!"),
            }
        }
        16 => {
            let matches = if star == 1 {
                aoc16::dance(&utils::merge_args(&args, 3, ","))
            } else {
                aoc16::dance_whole_night(&utils::merge_args(&args, 3, ","))
            };
            match matches {
                Some(matches) => println!("Dance {}!", matches),
                None => println!("Invalid input!"),
            }
        }
        17 => {
            let value = if star == 1 {
                aoc17::spinlock(&args[3])
            } else {
                aoc17::angry_spinlock(&args[3])
            };
            match value {
                Some(value) => println!("Value held by spinlock {}!", value),
                None => println!("Invalid input!"),
            }
        }
        18 => {
            if star == 1 {
                match aoc18::recover_frequency(&utils::merge_args(&args, 3, "\n")) {
                    Some(value) => println!("Last played sound {}!", value),
                    None => println!("Invalid input!"),
                }
            } else {
                match aoc18::count_sends(&utils::merge_args(&args, 3, "\n")) {
                    Some(send_count) => println!("Send request sent by program 1 {}!", send_count),
                    None => println!("Invalid input!"),
                }
            }
        }
        _ => println!("Unknown puzzle!"),
    }
}
