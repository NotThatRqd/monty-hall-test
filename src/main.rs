/*
 * MIT License
 *
 * Copyright (c) 2023 rad
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use user_input::{unsign64};

fn main() {
    let mut rng = thread_rng();

    println!("This program will simulate the Monty Hall problem");

    println!("How many iterations of the test would you like to run? (u32)");
    let iterations = unsign64();

    let mut wins = 0;

    println!("when the player switches:");
    for _i in 0..=iterations {
        if sim(true, &mut rng) {
            wins += 1;
        }
    }
    println!("{} wins out of {} tries", wins, iterations);

    // reset wins
    wins = 0;

    println!();

    println!("when the player doesn't switch:");
    for _i in 0..=iterations {
        if sim(false, &mut rng) {
            wins += 1;
        }
    }
    println!("{} wins out of {} tries", wins, iterations);

}

fn sim(should_switch: bool, rng: &mut ThreadRng) -> bool {
    let mut doors = [false, false, false];
    // set random door to winning door
    doors[rng.gen_range(0..=2)] = true;

    // get player's door
    let mut player_door = rng.gen_range(0..=2);

    // select a door that is not player_door and is false
    let mut monty_door = rng.gen_range(0..=2);
    while monty_door == player_door || doors[monty_door] == true {
        monty_door = rng.gen_range(0..=2);
    }

    // if player should switch, switch to the other door
    if should_switch {
        if 0 != player_door && 0 != monty_door {
            player_door = 0;
        } else if 1 != player_door && 1 != monty_door {
            player_door = 1;
        } else if 2 != player_door && 2 != monty_door {
            player_door = 2;
        }
    }

    // return if the player won
    doors[player_door]
}
