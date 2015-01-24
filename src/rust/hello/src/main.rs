//! ## About Meee
//! A simple program to help learning rust. It doesn't necessarily do anything useful.
//! **Der Weg ist das Ziel**

#![allow(unstable)]
extern crate mylib;

// This is a re-export that links to the public documentation automatically
#[doc(no_inline)]
pub use std::option::Option;

use std::io;
use std::os;
use std::rand;
use std::cmp::Ordering;

fn main() {
    let (mut x,mut y) = (0, 0);
    let mut out = std::io::stderr();

    for _ in 0..2 {
        out.write_line(format!("Hello, world: {}, {} !", x, y).as_slice()).ok().expect("Really have to print hello world");
        x = mylib::add_one(x);
        y = mylib::add_two(y);
    }

    const MAX: u32 = 100;
    const MIN: u32 = 1;
    let secret = (rand::random::<u32>() % MAX) + MIN;

    // this will be removed during release builds, whereas assert!() always remains
    debug_assert!(MIN <= secret);
    debug_assert!(secret <= MAX);

    let mut guess_count = 0;
    loop {

        print!("Guess a number between {} and {}\n$ ", MIN, MAX);
        let guess = io::stdin().read_line().ok().expect("Need stdin to work");
        let guess_trimmed = guess.trim();
        if guess_trimmed.len() == 0 {
            out.write_str("Error: no guess made\n").ok();
            os::set_exit_status(2);
            return;
        }
        
        // Inference by lhs type !
        let guess_number = match guess_trimmed.parse::<u32>() {
            Some(v) => v,
            None => { 
                out.write_line(format!("Couldn't understand your number {:?}", guess_trimmed).as_slice()).ok();
                os::set_exit_status(3);
                return;
            }
        };

        println!("You guessed: {}", guess_number);

        let res = cmp(guess_number, secret);
        println!("{}", match res {
            Ordering::Equal => "Correct !!!",
            Ordering::Less => "Secret was larger",
            Ordering::Greater => "Secret was smaller",
        });

        guess_count += 1;
        if res == Ordering::Equal {
            println!("You took {} guess{}", guess_count, if guess_count > 1 { "es" } else { "" });
            return;
        }
    }
}


fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
