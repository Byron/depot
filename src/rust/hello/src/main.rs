//! ## About Meee
//! A simple program to help learning rust. It doesn't necessarily do anything useful.
//! **Der Weg ist das Ziel**

extern crate mylib;

// This is a re-export that links to the public documentation automatically
#[doc(no_inline)]
pub use std::option::Option;


fn main() {
	let mut x = 0;
	for _ in 0..10 {
    	println!("Hello, world: {} !", x);
    	x = mylib::add_one(x);
	}
}

