use std::io::BufRead;

pub fn first () {
    let buffreader = super::puzzle_input::load_input(4);
    let mut lines: Vec<Vec<char>> = Vec::with_capacity(1000);
    let mut total = 0;
    for line in buffreader.lines().map(|l| l.unwrap()) {
        println!("-------------");
        println!("");
        lines.push(line.chars().collect());
    }
    println!("total: {:?}", lines);
}
// let next pointer = pass the word gotten by right
// no need to do left cause its redundant
// start point x or x
fn right () {

}
fn down() {

}
/*
I want to look downward left diagnol, down, right diagnol; starting from x or s (xmas | smax)
as well as in the same line : backward and foward (words can intersect each other
 */
/*
Ignore left if the space to the left is less than 4.
Ignore right if the space to the right is less than 4.
ignore bot if the space to the bot is less than 4.

 */