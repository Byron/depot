//! ## About Meee
//! A simple program to help learning rust. It doesn't necessarily do anything useful.
//! **Der Weg ist das Ziel**
#![feature(exit_status)]
#![allow(unstable)]
extern crate mylib;
extern crate rand;
// This is a re-export that links to the public documentation automatically
#[doc(no_inline)]
pub use std::option::Option;

use std::io::{self, Write, Read};
use std::env;
use std::cmp::Ordering;

fn main() {
    let (mut x,mut y) = (0, 0);
    let mut out = std::io::stderr();

    for _ in 0..2 {
        writeln!(out, "Hello, world: {}, {} !", x, y).unwrap();
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
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess_trimmed = guess.trim();
        if guess_trimmed.len() == 0 {
            out.write("Error: no guess made\n".as_bytes()).ok();
            env::set_exit_status(2);
            return;
        }
        
        // Inference by lhs type !
        let guess_number = match guess_trimmed.parse::<u32>() {
            Ok(v) => v,
            Err(e) => { 
                writeln!(out, "Couldn't understand your number {:?}", guess_trimmed).ok();
                env::set_exit_status(3);
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
