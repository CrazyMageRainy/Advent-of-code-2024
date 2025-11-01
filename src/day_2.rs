use std::io::BufRead;

pub fn main() {
    let buffreader = super::puzzle_input::load_input(2);
    let mut total = 0;
    for line in buffreader.lines().map(|l| l.unwrap()) {
        let numbers: Vec<_> = line
            .split_whitespace()
            // .map(|w| {println!("{}", w);
            // w})
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut stable = true;
        let mut direction: Option<bool> = None; //ascending true  decending: false
        let diff = numbers[1] - numbers[0];
        let abs_diff = i32::abs(diff);
        if abs_diff > 3 || abs_diff == 0 {
            continue;
        } else {
            direction.replace(diff > 0);
        }
        for i in 2..numbers.len() {
            let diff = numbers[i] - numbers[i - 1];
            let abs_diff = i32::abs(diff);
            if !(abs_diff <= 3 && diff != 0)
                || ((direction.unwrap() == false && diff > 0)
                    || (direction.unwrap() == true && diff < 0))
            {
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
enum Dir {
    Increase,
    Decrease,
}

pub fn second() {
    let buffreader = super::puzzle_input::load_input(2);
    let mut total = 0;
    for line in buffreader.lines().map(|l| l.unwrap()) {
        let numbers: Vec<_> = line
            .split_whitespace()
            // .map(|w| {println!("{}", w);
            // w})
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut stable = true;
        let mut direction: Option<bool> = None; //ascending true  decending: false
        let diff = numbers[1] - numbers[0];
        let abs_diff = i32::abs(diff);
        let mut s = 2;
        let mut dampened = false;

        if abs_diff > 3 || abs_diff == 0 {
            let new_diff = numbers[2] - numbers[0];
            let abs_diff = i32::abs(new_diff);
            dampened = true;
            if !(abs_diff < 3 && abs_diff != 0) {
                continue;
            } else {
                s = 3;
                direction.replace(new_diff > 0);
            }
        } else {
            direction.replace(diff > 0);
        }

        while s < numbers.len() {
            let diff = numbers[s] - numbers[s - 1];
            let abs_diff = i32::abs(diff);
            if !(abs_diff <= 3 && diff != 0)
                || ((direction.unwrap() == false && diff > 0)
                    || (direction.unwrap() == true && diff < 0))
            {
                if dampened == true {
                    stable = false;
                    break;
                } else {
                    if numbers.len() - s == 1 {
                        stable = false;
                        break;
                    }
                    dampened = true;
                    let new_diff = numbers[s + 1] - numbers[s - 1];
                    let abs_diff = i32::abs(new_diff);
                    if abs_diff > 3
                        || abs_diff == 0
                        || (direction.unwrap() == false && new_diff > 0)
                        || (direction.unwrap() == true && new_diff < 0)
                    {
                        stable = false;
                        break;
                    } else {
                        s += 2;
                        continue;
                    }
                }
            }
            s += 1;
        }

        if stable {
            total += 1;
        }
    }
    println!("total: {}", total);
}
