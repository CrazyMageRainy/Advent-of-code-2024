use std::io::stdin;
mod day_1;
mod day_2;
mod day_3;
mod puzzle_input;
mod day_4;

use day_1::*;
fn main() {
    println!("select the day you want to run: 1-25");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<u32>() {
        Ok(x) if x >= 1 && x <= 25 => {
            let day = format!("day_{}::main", x);
            // day_1::main();
            match x {
                1 => day_1::main(),
                2 => day_2::main(),
                3 => day_3::second(),
                4 => day_4::first(),
                // 5 => day 5::main(),
                // 6 => day 6::main(),
                // 7 => day 7::main(),
                // 8 => day 8::main(),
                // 9 => day 9::main(),
                // 10 => day 10::main(),
                // 2 => day 2::main(),
                // 2 => day 2::main(),
                // 2 => day 2::main(),
                // 2 => day 2::main(),
                // 2 => day 2::main(),
                // 2 => day 2::main(),
                // 2 => day 2::main(),
                // 2 => day 2::main(),
                _ => println!("not ready yet!"),
            }
        }
        Ok(x) => {
            eprintln!("Not a valid option");
        }
        _ => println!("The input was not a valid number"),
    }
}