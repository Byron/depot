
use std::thread;

#[test]
fn name() {
    // Add code here
}

#[test]
fn simple_threading() {
    let guard = thread::Thread::scoped(|| {
        panic!("I have failed");
    });

    // // convert to mutable - we own it, after all
    // // let mut guard = guard;
    // match guard.join() {
    //     Err(msg) => println!("{:?}", msg.unwrap()),
    //     _ => unreachable!(),
    // }

    // Iterator
}