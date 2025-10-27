use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main () {
    let p_file = File::open("./puzzle-input/day1.txt").unwrap();
    let mut buffreader = BufReader::new(p_file);
    let mut left_heap: BinaryHeap<u64> = BinaryHeap::with_capacity(1000);
    let mut right_heap: BinaryHeap<u64> = BinaryHeap::with_capacity(1000);

    for line in buffreader.lines().map(|l| l.unwrap()){
        println!("{:?}", line);
        let mut nums = line.trim().split_ascii_whitespace();
        // println!("{:?}", nums.peekable().peek());
        left_heap.push(nums.next().unwrap().parse::<u64>().unwrap());
        right_heap.push(nums.next().unwrap().parse::<u64>().unwrap());

        // break;
    }
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