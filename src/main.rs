use regex::Regex;
use rand::{thread_rng, Rng};
use std::env;

fn roll_dice(num_dice: u32, num_sides: u32, drop_action: char, modifier: i32) -> i32 {
    let mut rng = thread_rng();
    let mut dice = Vec::new();
    for _ in 0..num_dice {
        dice.push(rng.gen_range(1..num_sides + 1) as i32);
    }
    println!("drop: {}", drop_action);
    println!("rolls: {:?}", dice);
    if drop_action == 'L' {
        dice.sort();
        dice.reverse();
        dice.pop();
    } else if drop_action == 'H' {
        dice.sort();
        dice.pop();
    }
    println!("dice count: {}", dice.len());
    let result: i32 = dice.iter().sum();
    result + modifier
}

/// Takes a Dice Notation string and parses out the number of dice, number of sides and
/// modifications to be made with the roll 
fn parse_dice_notation(notation: &str) -> (u32, u32, char, i32) {
    let re = Regex::new(r"(\d+)d(\d+)([LH]?)([+-]?\d+)?").unwrap();
    let captures = re.captures(notation).unwrap();
    let num_dice = captures[1].parse().unwrap();
    let num_sides = captures[2].parse().unwrap();
    let drop_action = captures.get(3).map(|s| s.as_str()).unwrap_or(" ").chars().next().unwrap();
    let modifier = captures.get(4).map(|s| s.as_str().parse().unwrap()).unwrap_or(0);
    (num_dice, num_sides, drop_action, modifier)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: dice_roller <dice notation>");
        return;
    }
    let notation = &args[1];
    let (num_dice, num_sides, drop_action, modifier) = parse_dice_notation(notation);
    let result = roll_dice(num_dice, num_sides, drop_action, modifier);
    println!("Result: {}", result);
}
