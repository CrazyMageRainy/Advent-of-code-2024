use std::collections::{HashMap, HashSet};
use std::fs;
pub fn first() {
    // let file = File::open("./puzzle-input/day5.txt").unwrap();
    let file_string = fs::read_to_string("./puzzle-input/day5.txt").unwrap();

    let sections = file_string.split("\n\n").collect::<Vec<&str>>();

    // println!("{}", sections[1]);
    let mut before_dic: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(sections[0].len());

    for line in sections[0].split("\n") {
        let numbers: Vec<_> = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();
        before_dic
            .entry(numbers[0])
            .and_modify(|hs| {
                hs.insert(numbers[1]);
            })
            .or_insert(
                HashSet::from([numbers[1]]), //     {
                                             //     let mut hs = HashSet::new;
                                             //     hs.push(numbers[1]);
                                             //     hs
                                             // }
            );
    }
    // println!("{:?}", before_dic);
    let mut total = 0;
    for line in sections[1].split("\n") {
        let numbers: Vec<_> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        let middle = numbers.len() / 2;
        let mut correct_order = true;

        for (i, n) in numbers.iter().enumerate() {
            let mut j = i + 1;

            while j < numbers.len() {
                let before_num = before_dic.get(&numbers[j]).unwrap();
                if before_num.contains(n) {
                    correct_order = false;
                    break;
                }
                j += 1;
            }
            if !correct_order {
                break;
            }
        }
        if correct_order {
            total += numbers[middle];
        }
    }
    println!("{}", total);
}
