use regex::Regex;
use std::env;

mod roller;

/// Takes a Dice Notation string and parses out the number of dice, number of sides and
/// modifications to be made with the roll 
fn parse_dice_notation(notation: &str) -> (u32, u32, char, i32) {
    let re = Regex::new(r"(\d+)d(\d+)([LH]?)([+-]?\d+)?").unwrap();
    let captures = re.captures(notation).unwrap();
    let num_dice = captures[1].parse().unwrap();
    let num_sides = captures[2].parse().unwrap();
    let drop_action = captures.get(3).map(|s| s.as_str()).unwrap_or(" ").chars().next().unwrap_or('-');
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
    let result = roller::roll_dice(num_dice, num_sides, drop_action, modifier);
    println!("drop: {}", result.drop_action);
    println!("rolls: {:?}", result.individual_rolls);
    println!("dice count: {}", result.num_dice);
    println!("Result: {}", result.result_total);
}
