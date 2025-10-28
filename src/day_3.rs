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
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do|don't)\(\)").unwrap();
    // let input = "mul(23,123)";
    let extract_numbers = Regex::new(r"\d+").unwrap();
    let mut disabled = false;
    for line in buffreader.lines().map(|l| l.unwrap()) {
        let list: Vec<_> = re.find_iter(&line).collect();
        for e in list {
            println!("{:?}", e);
            match (e.as_str(), disabled) {
                ("do()", _) => disabled = false,
                ("don't()", _) => disabled = true,
                (_m, true) => continue,
                (m, false) => {
                    let extracted_numbers: Vec<_> = extract_numbers.find_iter(m).map(|x| x.as_str()).collect();
                    total += extracted_numbers[0].parse::<i32>().unwrap() * extracted_numbers[1].parse::<i32>().unwrap();
                }
            }
        }

        println!("total: {}", total);
    }
}
