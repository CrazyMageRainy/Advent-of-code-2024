use std::io::BufRead;

pub fn main() {
    let buffreader = super::puzzle_input::load_input(2);
    let mut total = 0;
    for line in buffreader.lines().map(|l| l.unwrap()) {
        let numbers: Vec<_> = line.split_whitespace()
            // .map(|w| {println!("{}", w);
            // w})
            .map(|n| n.parse::<i32>().unwrap()).collect();
        let mut stable = true;
        let mut direction: Option<bool> = None; //ascending true  decending: false
        for i in 1..numbers.len() {
            let diff = numbers[i] - numbers[i - 1];
            let abs_diff = i32::abs(diff);
            if  !(abs_diff <= 3 && diff != 0) && (direction.is_none() || direction.unwrap() == true ) {
                stable = false;
                break;
            }
        }
        if stable {
            total += 1;
        }
}
    println!("first total: {}", total);
}