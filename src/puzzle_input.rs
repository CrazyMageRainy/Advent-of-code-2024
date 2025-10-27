use std::fs::File;
use std::io::BufReader;

pub fn load_input(day: i32) -> BufReader<File>{
    let puzzle_file = format!("./puzzle-input/day{}.txt", day);
    let p_file = File::open(puzzle_file).unwrap();
    // let mut buffreader = BufReader::new(p_file);
    BufReader::new(p_file)
}