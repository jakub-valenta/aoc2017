mod aoc1;

use std::env;

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
        Ok(_) => println!("Unknown puzzle!"),
        Err(err) => println!("Invalid input {}!", err),
    }
}
