use rand::{thread_rng, Rng};
use std::env;

fn roll_dice(num_dice: u32, num_sides: u32, modifier: i32) -> i32 {
    let mut rng = thread_rng();
    let mut result = 0;
    for _ in 0..num_dice {
        result += rng.gen_range(1..num_sides + 1) as i32;
    }
    result += modifier;
    result
}

fn parse_dice_notation(notation: &str) -> (u32, u32, i32) {
    let parts: Vec<&str> = notation.split("d").collect();
    let num_dice = parts[0].parse().unwrap();
    let mut num_sides = 0;
    let mut modifier: i32 = 0;
    if let Some(index) = parts[1].find('+') {
        num_sides = parts[1][..index].parse().unwrap();
        modifier = parts[1][index + 1..].parse().unwrap();
    } else if let Some(index) = parts[1].find('-') {
        num_sides = parts[1][..index].parse().unwrap();
        modifier = -1 * parts[1][index + 1..].parse().unwrap();
    } else {
        num_sides = parts[1].parse().unwrap();
    }
    (num_dice, num_sides, modifier)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: dice_roller <dice notation>");
        return;
    }
    let notation = &args[1];
    let (num_dice, num_sides, modifier) = parse_dice_notation(notation);
    let result = roll_dice(num_dice, num_sides, modifier);
    println!("Result: {}", result);
}
