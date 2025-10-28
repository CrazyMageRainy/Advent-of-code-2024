use std::collections::VecDeque;
use std::io::BufRead;
use regex::Regex;
pub fn main() {
    let buffreader = super::puzzle_input::load_input(3);
    let mut total = 0;
    // let mut pointer = 0;
    // let re = Regex::new("mul\\((\\d{0,3}),\\d{0,3}\\)").unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for line in buffreader.lines().map(|l| l.unwrap()) {
        println!("-------------");
        println!("");
        // let mut line_chars: VecDeque<char> = line.chars().collect();
        // let digits = 3;
        // // let length = line.len();
        // //ex: mul(123,2) max 3 digits
        // // while pointer < length {
        // //     if line_chars[pointer] == 'm' {
        // //
        // //     }
        // // }
        // while let Some(char) = line_chars.pop_front() {
        //
        // }
        for (complete_mul, [a, b]) in re.captures_iter(&line).map(|c| c.extract()) {
            println!("a:{} b: {} | complete_mul: {}", a, b, complete_mul);
            let int_a = a.parse::<u32>().unwrap();
            let int_b = b.parse::<u32>().unwrap();
                total += int_a * int_b;
            // println!("{}", complete_mul);
        }
        // for c in instances {
        //     println!("{:?}", c);
        // }
        println!("total: {}", total);
    }
}
pub fn second() {
    let buffreader = super::puzzle_input::load_input(3);
    let mut total = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\) | (do|don't)\(\)").unwrap();
    for line in buffreader.lines().map(|l| l.unwrap()) {
        for (complete_mul, [a, b]) in re.captures_iter(&line).map(|c| c.extract()) {
            println!("a:{} b: {} | complete_mul: {}", a, b, complete_mul);
            // let int_a = a.parse::<u32>().unwrap();
            // let int_b = b.parse::<u32>().unwrap();
            // total += int_a * int_b;
            // println!("{}", complete_mul);
        }
        println!("hi");
    }
}
