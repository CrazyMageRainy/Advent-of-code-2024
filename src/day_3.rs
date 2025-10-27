use std::collections::VecDeque;
use std::io::BufRead;
use regex::Regex;
pub fn main() {
    let buffreader = super::puzzle_input::load_input(3);
    let total = 0;
    let mut pointer = 0;
    let re = Regex::new("mul\\((\\d{0,3}),\\d{0,3}\\)").unwrap();
    for line in buffreader.lines().map(|l| l.unwrap()) {
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
        for (complete_mul, [b]) in re.captures_iter(&line).map(|c| c.extract()) {

        }
        // for c in instances {
        //     println!("{:?}", c);
        // }

    }
}