#![allow(unstable)]

use std::{io,vec};

static LONG_LIVED: &'static str = "foo";

#[test]
fn looping() {

    // FOR 
    let mut c = 0;
    for _ in 0..10 {
        c += 1;
    }
    assert!(c == 10);

    // endless LOOP
    let mut c = 1;
    loop {
        if c % 10 == 0 {
            break
        }
        c += 1;
    }
    assert!(c == 10);

    let mut c = 1;
    // WHILE
    while c % 10 != 0 {
        c += 1;
    }
}

#[test]
fn compound_data() {
    struct Point {
        x: i32,
        y: i32,
    }

    let mut p = Point{x: 0, y: 0};
    let mut o = Point{x: p.x, y: p.y};

    p.x = 5;
    o.x = 10;
    assert!(o.x != p.x);


    struct PointT(i32, i32);

    let p = PointT(5, 10);
    assert!(p.0 == 5 && p.1 == 10);


    struct Inches(f32);
    let i = Inches(25.0);

    assert!(i.0 == 25.0);
}

#[test]
fn enumerations() {
    #[derive(PartialEq)]
    enum Order {
        Less,
        Equal,
        Greater
    }

    fn cmp(x: i32, y: i32) -> Order {
        if x < y { Order::Less }
        else if x > y { Order::Greater }
        else { Order::Equal }
    }

    let o = cmp(1, 10);
    assert!(o == Order::Less);
    let res = match o {
        Order::Greater => "g",
        Order::Less => "l",
        Order::Equal => "e",
    };
    assert!(res == "l");

    #[derive(PartialEq)]
    enum OptionalInt {
        Value(i32),
        Nothing
    }

    let m = OptionalInt::Value(30);
    let n = match m {
        OptionalInt::Value(r) => r,
        OptionalInt::Nothing => 0,
    };
    assert!(n == 30);
    assert!(!(m == OptionalInt::Nothing));
}

#[test]
fn strings() {
    let s = "Hi";                   // This is a string slice
    assert!(s.len() == 2);

    let si = s.to_string();
    assert!(si.len() == s.len());
    assert!(si.ends_with("i"));

    // Actually, replace is read-only as it returns a new string
    // assert!(si.replace("Hi", "Ho").as_slice() == "Ho")
    assert!(s.replace("Hi", "Ho").as_slice() == "Ho")
}

#[test]
#[should_fail]
fn no_static_inference() {
    // Compiler cannot see at compile time that this wouldn't work
    // First I put in a signed integer, which gets converted to a unsigned
    // Second, it doesn't realize that the index is  out of bounds in any case
    let b = ["a", "b", "c"];
    assert!(b[-1] == "c");
}

#[test]
fn arrays() {
    let a = [1,2,3];
    assert!(a[0] == 1);

    let mut b = ["a", "b"];
    b[1] = LONG_LIVED;
    assert!(b[1] == LONG_LIVED);

    for item in b.iter() {
        println!("{}", item);
    }

    // compile time constant !
    const S: usize = 20;
    let mut a = [5; S];
    assert!(a.len() == S);

    // This scope is needed as we have an immutable borrow
    // but want to borrow a mutable later
    {
        let sl: &[i32] = a.slice(1,2);  // type given just to show it ... useful for fn ()
        assert!(sl.len() == 1);
        assert!(sl[0] == a[1]);
    }

    let sl2 = &mut a[0..5];
    assert!(sl2.len() == 5);
    sl2[0] = 0;
    assert!(sl2[0] == 0);
}

#[test]
fn vectors() {
    // it's convention to use vec![] instead of vec!()
    let b = vec![1,2,3];
    assert!(b.len() == 3);
    for item in b.into_iter() {
        println!("{:?}", item);
    }
    // Can't use b anymore ! into_iter() takes ownership out of vector
    // assert!(b.len() == 0);

    // vec!() is also possible - it's just how macros can be called in general
    let mut b = vec!(1,2);
    assert!(b.len() == 2);
    b.push(3);
    assert!(b.len() == 3);

    struct Single {
        stream: io::LineBufferedWriter<io::stdio::StdWriter>
    };

    // It seems structs are being mem-copied, ownership is not transferred.
    // How to deal with resource allocation.
    // This actually works
    let mut v = Single{stream: io::stderr()};
    let mut b = vec::Vec::new();
    b.push(v);

    // v was moved, an even though assign works ... 
    v.stream = io::stdout();
    // you can't access a moved v ... is that supposed to be like that ?
    // The cool thing is that members make their parent structs non-copyable automatically.
    // v.stream.write(b"hello");

    struct SingleInt {
        x: i32
    }


    let mut b = vec::Vec::new();
    let mut v = SingleInt{x: 3};
    b.push(v);
    v.x = 4;
    assert!(b[0].x == 3);
}
