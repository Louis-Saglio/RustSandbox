extern crate core;

use std::cmp::min;

use rand::prelude::SliceRandom;

use crate::Player::{ATTACKER, DEFENDER};

const ATTACKERS_MAX_DICE_NBR: u32 = 3;
// const DEFENDERS_MAX_DICE_NBR: u32 = 2;
const DICE_FACES: [u8; 6] = [1, 2, 3, 4, 5, 6];

#[derive(Debug)]
struct Stats {
    attacker_winning_rate: f64,
    average_attacker_units_remaining: f64,
    average_defender_units_remaining: f64,
}

#[derive(Debug)]
enum Player {
    ATTACKER, DEFENDER,
}

#[derive(Debug)]
struct BattleResult {
    winner: Player,
    remaining_attackers_nbr: u32,
    remaining_defenders_nbr: u32,
}

impl BattleResult {
    fn to_string(&self) -> String {
        let name = match self.winner {
            ATTACKER => "Attacker",
            DEFENDER => "Defender",
        };
        let remaining_nbr = match self.winner {
            ATTACKER => self.remaining_attackers_nbr,
            DEFENDER => self.remaining_defenders_nbr,
        };
        format!("{} wins with {} unit(s) remaining", name, remaining_nbr)
    }
}

fn throw_dice() -> u8 {
    *DICE_FACES.choose(&mut rand::thread_rng()).unwrap()
}

fn compute_defender_dice_number_0(attacker_dice: &Vec<u8>, attackers_nbr: &u32, defenders_nbr: &u32) -> u32 {
    return if defenders_nbr == &1 {
        1
    } else {
        2
    }
}

fn compute_defender_dice_number_1(attacker_dice: &Vec<u8>, attackers_nbr: &u32, defenders_nbr: &u32) -> u32 {
    if defenders_nbr == &1 {
        return 1;
    }
    let sum: u8 = attacker_dice[..min(2, attacker_dice.len())].iter().sum();
    if sum >= 8 { 1 } else { 2 }
}

fn compute_defender_dice_number_2(attacker_dice: &Vec<u8>, attackers_nbr: &u32, defenders_nbr: &u32) -> u32 {
    if defenders_nbr == &1 {
        return 1;
    }
    if defenders_nbr == &2 {
        return 2;
    }
    let sum: u8 = attacker_dice[..min(2, attacker_dice.len())].iter().sum();
    if sum >= 8 { 1 } else { 2 }
}

fn compute_defender_dice_number_3(attacker_dice: &Vec<u8>, attackers_nbr: &u32, defenders_nbr: &u32) -> u32 {
    if defenders_nbr == &1 {
        return 1;
    }
    if defenders_nbr == &2 {
        return 2;
    }
    if attackers_nbr <= &3 {
        return 2;
    }
    let sum: u8 = attacker_dice[..min(2, attacker_dice.len())].iter().sum();
    if sum >= 8 { 1 } else { 2 }
}

fn risk(initial_attackers_nbr: &u32, initial_defenders_nbr: &u32) -> BattleResult {
    let mut attackers_nbr = *initial_attackers_nbr;
    let mut defenders_nbr = *initial_defenders_nbr;
    loop {
        if attackers_nbr == 1 || defenders_nbr == 0 {
            break
        }
        let a_dice = {
            let dice_nbr = min(ATTACKERS_MAX_DICE_NBR, attackers_nbr - 1);
            let mut a_dice: Vec<u8> = Vec::new();
            for _ in 0..dice_nbr {
                a_dice.push(throw_dice())
            }
            a_dice.sort();
            a_dice.reverse();
            a_dice
        };
        let d_dice = {
            let dice_nbr = compute_defender_dice_number_0(&a_dice, &attackers_nbr, &defenders_nbr);
            let mut d_dice: Vec<u8> = Vec::new();
            for _ in 0..dice_nbr {
                d_dice.push(throw_dice())
            }
            d_dice.sort();
            d_dice.reverse();
            d_dice
        };
        for (d_single_dice, a_single_dice) in d_dice.iter().zip(&a_dice) {
            if d_single_dice >= a_single_dice {
                attackers_nbr -= 1;
            } else {
                defenders_nbr -= 1;
            }
        }
    }
    if attackers_nbr == 1 {
        BattleResult {
            winner: DEFENDER,
            remaining_attackers_nbr: attackers_nbr,
            remaining_defenders_nbr: defenders_nbr,
        }
    } else if defenders_nbr == 0 {
        BattleResult {
            winner: ATTACKER,
            remaining_attackers_nbr: attackers_nbr,
            remaining_defenders_nbr: defenders_nbr,
        }
    } else {
        panic!()
    }
}

fn compute_risk_battle_stats(attackers_nbr: &u32, defenders_nbr: &u32, n: &u32) -> Stats {
    let mut total_a_units_remaining = 0u32;
    let mut total_d_units_remaining = 0u32;
    let mut total_a_wins = 0u32;
    for _ in 0..*n {
        let result = risk(&attackers_nbr, &defenders_nbr);
        total_a_units_remaining += result.remaining_attackers_nbr;
        total_d_units_remaining += result.remaining_defenders_nbr;
        if let ATTACKER = result.winner {
            total_a_wins += 1;
        }
    };
    Stats {
        attacker_winning_rate: (total_a_wins as f64) / (*n as f64),
        average_attacker_units_remaining: (total_a_units_remaining as f64) / (*n as f64),
        average_defender_units_remaining: (total_d_units_remaining as f64) / (*n as f64),
    }
}

fn main() {
    println!("{:?}", compute_risk_battle_stats(&12, &12, &10_000_000));
}
