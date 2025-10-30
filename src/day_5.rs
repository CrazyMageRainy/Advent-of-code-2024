use std::collections::{HashMap, HashSet};
use std::fs;
pub fn first() {
    // let file = File::open("./puzzle-input/day5.txt").unwrap();
    let file_string = fs::read_to_string("./puzzle-input/day5.txt").unwrap();

    let sections = file_string.split("\n\n").collect::<Vec<&str>>();

    // println!("{}", sections[1]);
    let before_dic: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(sections[0].len());

    for line in sections[0].split("\n") {
        let numbers = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
        before_dic.entry(numbers[0]).and_modify(|hs| hs.push(numbers[1])).or_insert({
            let hs = HashSet::new;
            hs.push(numbers[1]);
            hs
        });
    }

}