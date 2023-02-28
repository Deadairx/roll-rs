use rand::{thread_rng, Rng};
use std::fmt::Display;

pub enum DropAction {
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

pub struct RollResult {
    pub num_dice: u32,
    pub sides: u32,
    pub drop_action: DropAction,
    pub individual_rolls: Vec<u32>,
    pub modifier: i32,
    pub result_total: i32,
}

impl Default for RollResult {
    fn default() -> Self {
        Self {
            num_dice: 0,
            sides: 0,
            drop_action: DropAction::Keep,
            individual_rolls: Vec::new(),
            modifier: 0,
            result_total: 0,
        }
    }
}

pub fn roll_dice(num_dice: u32, num_sides: u32, drop_action: char, modifier: i32) -> RollResult {
    let mut rng = thread_rng();
    let mut dice = Vec::new();
    for _ in 0..num_dice {
        dice.push(rng.gen_range(1..num_sides + 1) as i32);
    }

    let drop_action = match drop_action {
        'L' => DropAction::DropLowest,
        'H' => DropAction::DropHighest,
        _ => DropAction::Keep,
    };

    //if drop_action == 'L' {
        //dice.sort();
        //dice.reverse();
        //dice.pop();
    //} else if drop_action == 'H' {
        //dice.sort();
        //dice.pop();
    //}

    let drop_amount = match drop_action {
        DropAction::DropLowest => dice.iter().min().unwrap_or(&0),
        DropAction::DropHighest => dice.iter().max().unwrap_or(&0),
        DropAction::Keep => &0
    };

    let result: i32 = (dice.iter().sum::<i32>()) - *drop_amount;
    let result_total = result + modifier;

    RollResult {
        num_dice,
        sides: num_sides,
        drop_action,
        individual_rolls: dice.iter().map(|x| *x as u32).collect(),
        modifier,
        result_total,
    }
}

