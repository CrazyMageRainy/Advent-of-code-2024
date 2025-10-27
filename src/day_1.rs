use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use super::puzzle_input;

// use crate::puzzle_input::load_input;
pub fn main () {
    let p_file = File::open("./puzzle-input/day1.txt").unwrap();
    // let mut buffreader = BufReader::new(p_file);
    let buffreader = puzzle_input::load_input(1);
    let mut left_heap: BinaryHeap<u64> = BinaryHeap::with_capacity(1000);
    let mut right_heap: BinaryHeap<u64> = BinaryHeap::with_capacity(1000);

    for line in buffreader.lines().map(|l| l.unwrap()){
        // println!("{:?}", line);
        let mut nums = line.trim().split_ascii_whitespace();
        // println!("{:?}", nums.peekable().peek());
        left_heap.push(nums.next().unwrap().parse::<u64>().unwrap());
        right_heap.push(nums.next().unwrap().parse::<u64>().unwrap());

        // break;
    }
    sec_question(&mut left_heap, &mut right_heap);
    return;
    // println!("Done adding to heaps");
    // println!("{:?}", left_heap.peek());
    // println!("{:?}", right_heap.peek());
    let mut total = 0;
    //part 1
    // idk why while let (a,b) = left_heap.iter().zip(right_heap.iter().next() failed :(
    // for (a, b) in  *left_heap.max().iter().zip(right_heap.into_iter()) {
        // let (a, b) = if a < b {(b, a)} else {(a, b)};
    while let Some(a) = left_heap.pop() && let Some(b) = right_heap.pop(){
        println!("a:{} b:{}", a ,b);
        if a < b {
            total += b - a;
        } else {
            total += a - b;
        }
    }
    // let p_input = fs::File::read(p_file);
    println!("total: {}", total);
}

fn sec_question(left: &mut BinaryHeap<u64>, right: &mut BinaryHeap<u64>) {
    let mut similarity_score = 0;
    let mut right_nums: HashMap<u64, u64> = HashMap::with_capacity(right.len());
    while let Some(b) = right.pop() {
        right_nums.entry(b).and_modify(|mut amount| *amount += 1).or_insert(1);
    }
    while let Some(a) = left.pop() {
        match right_nums.get(&a) {
            Some(x) => similarity_score += (x * a),
            None => (),
        }
    }
    println!("sim_score:{}", similarity_score);
}