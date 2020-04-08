mod game;

use crate::game::{Game, Player};

fn main() {
    let game = Game { p1: Player::default(), p2 :Player::default() };
    
    println!("Hello, world!");
}
