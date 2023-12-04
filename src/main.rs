mod pokemon;
use std::{io::stdin, process::exit};

use crate::pokemon::{IMPLEMENTED_PKMN, PKMN_NAMES};

type Pkmn = u8;

fn main() {
    println!("Welcome to the Pokèmon game!!!");

    let red_pkmn = choose_red_pkmn();
    let green_pkmn = choose_green_pkmn();

    println!("Starting a battle between Red and Green");
}

fn choose_red_pkmn() -> Pkmn {
    let pkmn;
    println!("Chose the Red's Pokèmon: [1..151]");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Could not read the chosen Pokèmon");
    pkmn = buffer
        .trim()
        .parse::<Pkmn>()
        .expect("Please insert a number")
        - 1;
    if pkmn as usize >= IMPLEMENTED_PKMN {
        println!("Pokèmon not yet implemented");
        exit(-1);
    } else {
        println!(
            "Red chose the Pokèmon n. {buffer} {}",
            PKMN_NAMES[pkmn as usize]
        );
    }
    pkmn
}

fn choose_green_pkmn() -> Pkmn {
    let pkmn;
    println!("Chose the Green's Pokèmon: [1..151]");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Could not read the chosen Pokèmon");
    pkmn = buffer
        .trim()
        .parse::<Pkmn>()
        .expect("Please insert a number")
        - 1;
    if pkmn as usize >= IMPLEMENTED_PKMN {
        println!("Pokèmon not yet implemented");
        exit(-1);
    } else {
        println!(
            "Green chose the Pokèmon n. {buffer} {}",
            PKMN_NAMES[pkmn as usize]
        );
    }
    pkmn
}
