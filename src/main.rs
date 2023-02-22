use regex::Regex;
use rand::{thread_rng, Rng};
use std::env;
use std::fmt::Display;

enum DropAction {
    DropHighest,
    DropLowest,
    Keep,
}

impl Display for DropAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DropAction::DropHighest => write!(f, "DropHighest"),
            DropAction::DropLowest => write!(f, "DropLowest"),
            DropAction::Keep => write!(f, "Keep"),
        }
    }
}

struct RollResult {
    num_dice: u32,
    sides: u32,
    drop_action: DropAction,
    individual_rolls: Vec<u32>,
    modifier: i32,
    result_total: i32,
}

fn roll_dice(num_dice: u32, num_sides: u32, drop_action: char, modifier: i32) -> RollResult {
    let mut rng = thread_rng();
    let mut dice = Vec::new();
    for _ in 0..num_dice {
        dice.push(rng.gen_range(1..num_sides + 1) as i32);
    }
    if drop_action == 'L' {
        dice.sort();
        dice.reverse();
        dice.pop();
    } else if drop_action == 'H' {
        dice.sort();
        dice.pop();
    }
    let result: i32 = dice.iter().sum();
    let result_total = result + modifier;

    RollResult {
        num_dice,
        sides: num_sides,
        drop_action: match drop_action {
            'L' => DropAction::DropLowest,
            'H' => DropAction::DropHighest,
            _ => DropAction::Keep,
        },
        individual_rolls: dice.iter().map(|x| *x as u32).collect(),
        modifier,
        result_total,
    }
}

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
    let result = roll_dice(num_dice, num_sides, drop_action, modifier);
    println!("drop: {}", result.drop_action);
    println!("rolls: {:?}", result.individual_rolls);
    println!("dice count: {}", result.num_dice);
    println!("Result: {}", result.result_total);
}
