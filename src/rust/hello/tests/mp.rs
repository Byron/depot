
use std::thread;
use std::sync::mpsc::channel;

#[test]
fn name() {
    // Add code here
}

#[test]
fn simple_threading() {
    {
        let guard = thread::Thread::scoped(|| {
            panic!("I have failed");
        });
    
        assert_eq!(guard.thread().name().unwrap_or("NO NAME"), "NO NAME");
    
        // guard ceases to exist right here, and joins
    }

    // Ownership transfer
    {
        // Channels
        let (tx, rx) = channel::<&str>();
        thread::Thread::scoped(move || {
            for x in range(0, 10) { tx.send("hi"); }
        });

        let mut c = 0;
        for res in rx.iter() {
            // Match would be nicer !!!
            assert_eq!(res, "hi");
            c += 1;
        }

        assert!(c == 10);
    }

    // Select and timers ... I know select! can be used to do the same thing 
    // as in go.

    // Maybe unsafe cells ?
    // Shall be done life !
}