#![allow(dead_code, unused_variables, unused_assignments, deprecated)]
#![feature(box_syntax, slice_patterns)]

extern crate rustc_serialize;
extern crate chrono;

use std::fmt;
use std::iter;

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

    let mut p = Point { x: 0, y: 0 };
    let mut o = Point { x: p.x, y: p.y };

    p.x = 5;
    o.x = 10;
    assert!(o.x != p.x);


    struct PointT(i32, i32);

    let p = PointT(5, 10);
    assert!(p.0 == 5 && p.1 == 10);


    struct Inches(f32);
    let i = Inches(25.0);

    assert_eq!(i.0, 25.0);
}

#[test]
fn enumerations() {
    #[derive(PartialEq)]
    enum Order {
        Less,
        Equal,
        Greater
    }

    impl Order {
        fn instance_method(&self) -> &'static str {
            match *self {
                Order::Less => "Less",
                Order::Equal => "Equal",
                Order::Greater => "Greater",
            }
        }
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
    let s = "Hi";             // This is a string slice
    // warning: use of deprecated item: use std::convert::AsRef<str> instead, #[warn(deprecated)] on by default
    assert!(&s.replace("Hi", "Ho") == "Ho");
    // tests/lang.rs:120:35: 120:43 error: type annotations required: cannot resolve `collections::string::String : core::convert::AsRef<_>` [E0283]
    assert!(s.replace("Hi", "Ho") == "Ho");
}

#[test]
#[should_panic]
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
        let sl = &a[1 .. 2];  // type given just to show it ... useful for fn ()
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
    let mut b = vec!(1, 4);
    assert_eq!(b.len(), 2);
    assert_eq!(b[0], 1);
    b.push(3);
    assert!(b.len() == 3);


    // you can't access a moved v ... is that supposed to be like that ?
    // The cool thing is that members make their parent structs non-copyable automatically.
    // v.stream.write(b"hello");

    struct SingleInt {
        x: i32
    }


    let mut b = Vec::new();
    let mut v = SingleInt{x: 3};
    b.push(v);
    v.x = 4;
    assert!(b[0].x == 3);

    // preallocate
    let c: Vec<u8> = (0u8 .. 255u8).collect();
    assert_eq!(c.len(), 255);

    let c: Vec<usize> = iter::repeat(0).take(500).collect();
    assert_eq!(c.len(), 500);
}

#[test]
#[allow(dead_code)]
#[allow(unused_variables)]
fn ownership() {
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Car {
        name: String,
    }

    #[derive(Copy, Clone)]
    struct Wheel<'a> {
        owner: &'a Car,
    }

    static NUM_WHEELS: u8 = 4;
    let car = Car { name: "DeLorean".to_string() };

    for _ in 0 .. NUM_WHEELS {
        Wheel { owner: &car };
    }

    // Borrows are always fine - they are read-only !
    let wheels = [Wheel { owner: &car }; 4];

    let mut dyn_wheels = Vec::with_capacity(NUM_WHEELS as usize);
    for _ in 0 .. NUM_WHEELS {
        dyn_wheels.push(box Wheel { owner: &car });
    }

    let ocar = Rc::new(RefCell::new(Car { name: "Trabant".to_string() }));


    // Mutable reference only really work with RefCell and refcounts in this case
    // also means the car needs heap allocation, even though stack allocation should
    // be fine ... is there a better way to this ?
    struct OWheel {
        owner: Rc<RefCell<Car>>,
    }

    let mut wheels = Vec::new();
    for _ in 0 .. NUM_WHEELS {
        wheels.push(OWheel { owner: ocar.clone() });
    }

    // Car with replaceable wheels and backpointer of wheel to car
    struct OCar<'a> {
        wheels: [Wheel<'a>; 4]
    }

    // // // TODO: WITH Generics on Wheel !! Car should be OCar here 
    // impl<'a> OCar<'a> {
    //     fn replace_wheel(&'a mut self, wheel_id: usize) -> Wheel {
    //         let mut previous = self.wheels[wheel_id];
    //         self.wheels[wheel_id] = Wheel { owner: self };
    //         previous
    //     }
    // }
}

#[test]
#[allow(unused_variables)]
fn advanced_patterns() {

    let x = 3;
    match x {
        1 | 2 => assert!(false, "Shouldn't be here"),
        _ => (),
    }

    match x {
        e @ 0 ... 3 => { format!("x is between 0 and incl. 3: {}", e); () },
        _ => unreachable!(),
    }

    let x = Some(5);
    match x {
        Some(..) => (),
        None => unreachable!(),
    }

    // pattern guards use if
    match x {
        Some(v) if v == 5 => (),
        _ => unreachable!()
    }

    // destructuring pattern (implicit derefenece ... kind of)
    let y: &i32 = &x.unwrap();
    match y {
        &y => (),   // note that here x, is dereferenced, so we get a value // i32
    }

    // You can ask for a ref explicitly though
    match x {
        ref z => (), // &i32
    }

    let mut x = 5;
    match x {
        ref mut z => (), //&mut i32
    }

    struct Point {
        x: i32,
        y: i32,
    }

    // Destructure members of struct
    let mut p = Point {x: 1, y: 2};
    match p {
        Point { x: u, y: v } => p.x += p.y,
    }
    assert!(p.x == 3);

    // Destruction only some members
    match p{
        // without ref mut, we would get a copy of the POD
        Point { y: ref mut u, .. } => {*u += 5; ()}, 
    }
    assert!(p.y == 7);

    match p {
        ref mut m if m.x == 3 => m.x += 3,
        // m => (), this would move p, so it can't be used anymore
        _ => assert!(false),
    }
    assert!(p.x == 6);

    // you can also define you want a borrow right away
    match &mut p {
        m => m.x += 3,
    }

    // decomposition cannot be nested, so Some(Point(x: x, y: y)) won't work
    let mut p = Some(Point { x: 5, y: 10 } );
    match p {
        Some(ref mut p) => { p.x += p.y; () },  // p is &Point, *p.x is a way to access it
        None => assert!(false),
    }
    assert!(p.unwrap().x == 15);


    // Tuple destucturing !
    let mut t = (5, "foo", 24.0);

    match t {
        (ref mut x, _, _) if *x == 5 => *x += 5,
        (_, x, _) if x == "bar" => (),
        (_, _, _) => (),
    }
    assert!(t.0 == 10);

    // Array destructuring
    // NOTE: For arrays with any unkonwn size, .. works !
    // Tuples require you to do 
    let mut a = [1, 2, 3];
    match &mut a {
        &mut [ref mut u, ref mut v, _] if *u == 1 && *v == 2 => { 
            *u += 3; 
            *v += 2 },
        &mut [..] => (),
        // _ => (), would be equivalent
    }
    assert!(a[0] == a[1]);

}

#[test]
fn methods() {

    const FROM_BASE: f32 = 2.54;

    #[derive(PartialEq)]
    struct Centimeters(f32);
    struct Inches(f32);

    impl Centimeters {
        fn from_inches(inch: &Inches) -> Centimeters {
            inch.to_cm()
        }
    }

    impl Inches {
        fn from_cm(cm: &Centimeters) -> Inches {
            Inches(cm.0 * FROM_BASE)
        }

        fn to_cm(&self) -> Centimeters {
            Centimeters(self.0 / FROM_BASE)
        }
    }

    let cm = Centimeters(20.0);
    let inch = Inches::from_cm(&cm);

    assert!(cm.0 == 20.0);
    assert!(cm == inch.to_cm());

    let cm = Centimeters::from_inches(&inch);
    assert!(cm == inch.to_cm());

    // deconstruction/matching of a NewType
    let Centimeters(myone) = cm;
    assert!(myone == cm.0);
}

#[test]
fn closures() {
    // note that THIS doesnt work: x + 1 - type inference fails I guess
    // borrow closure for read-only access: |&:|
    let add_one_ref = |x| { 1 + x };
    let explicit_add_one_ref = |x: i32| { x + 1 };
    let x = 1;

    assert!(add_one_ref(x) == 2);
    assert!(x == 1);
    assert!(explicit_add_one_ref(x) == 2);
    assert!(x == 1);

    // mutable borrow closure returns ownership after so: |&mut|
    let mut x = 1;
    {
        let mut add_one_to_x = || -> i32 { x += 1; x };
        assert!(add_one_to_x() == 2);
        // assert!(add_one_to_x() == x); // doesn't work because x is still borrowed mutably
        // assert!(2 == x);             // same here ... :)
        // x = 2;                       // no writing !
    }
    
    assert!(x == 2);
    x = 3;   // can adjust it again
    let x = x; // drop mutability
    assert!(x == 3);
    // x = 5;  // this fails now !

    // TODO: MOVING CLOSURE
    // let mut x = 1;
    {
        // let add_one_to_x = || { x += 1 };
        // add_one_to_x();
        // assert!(x == 2);
    }
    // assert!(x == 2);


    let x = 3;
    fn thrice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
        f(x) * 3
    }

    fn subtract_one(x: i32) -> i32 {
        x - 1
    }

    assert_eq!(thrice(x, |x| { x - 1 }), 6);
    assert!(x == 3);
    assert_eq!(thrice(x, subtract_one), 6);
}

#[test]
fn iterators() {
    let mut range_10 = 0 .. 10;
    let mut c = 0usize;
    loop {
        match range_10.next() {
            Some(_) => (),
            None => break,
        }

        assert!(c < 10);
        c += 1;
    }

    let one_to_hundred = (1 .. 101).collect::<Vec<i32>>();
    assert_eq!(one_to_hundred.len(), 100);

    let one_to_fifty = (1 .. std::usize::MAX).take(50).collect::<Vec<usize>>();
    assert!(one_to_fifty.len() == 50);
}

#[test]
fn sizes() {

    assert_eq!(std::mem::size_of::<i32>(), 4);
    assert_eq!(std::mem::size_of::<Option<i64>>(), 16);
    assert_eq!(std::mem::size_of::<Option<i32>>(), 8);
    assert_eq!(std::mem::size_of::<Result<u8, bool>>(), 2);

    #[allow(dead_code)]
    enum Multiple {
        One(bool),
        Two(i32),
        Three(i64)
    }

    assert!(std::mem::size_of::<Multiple>() == 16);
}

#[test]
#[allow(unused_variables)]
fn generics_and_traits() {
    let v: Option<i32> = Some(5);


    trait Outspoken : fmt::Debug {};

    // This defines a default implementation to anything with the Outspoke trait.
    trait OutspokenImpl : Outspoken {
        fn speak(&self) -> String {
            format!("{:?}", self)
        }
    }

    // This line tells the generics system to provide the implementation to all types
    // which are outspoken
    impl<T> OutspokenImpl for T where T: Outspoken {}

    #[derive(Debug)]
    struct MyType(i32);

    // Add Outspoken marker to my type
    // impl Outspoken for MyType {};
    // Add Outspoken to all types that have the Debug trait
    impl<T> Outspoken for T where T: fmt::Debug {}


    assert_eq!(format!("{:?}", MyType(15)), "MyType(15)");
    let mti = MyType(20);
    assert_eq!(mti.speak(), "MyType(20)");

    // You can bark even though the implementation follows later.
    // Makes sense as we handle generics at compile time
    assert_eq!(mti.bark(), "wuff");

    // Add your own methods to any existing type who is Outspoken
    trait AmendDoggyness : Outspoken {
        fn bark(&self) -> &str {
            "wuff"
        }
    }

    impl<T> AmendDoggyness for T where T: Outspoken {}
}

#[test]
fn outspoken_from_stackoverflow() {
    trait Outspoken {
        fn speak(&self) -> String;
    }

    impl<T> Outspoken for T where T: fmt::Debug {
        fn speak(&self) -> String {
            format!("{:?}", self)
        }
    }

    #[derive(Debug)]
    struct MyType(i32);

    fn main() {
        assert_eq!(format!("{:?}", MyType(15)), "MyType(15)");
        assert_eq!(MyType(20).speak(), "MyType(20)");
    }
}


#[test]
fn type_aliases() {
    // type actually declares something like a new type with given memory layout
    struct Foo {
        a: u32,
    };
    impl Foo {
        fn bark(&self) {
            println!("Wuff {}", self.a);
        }

        fn id() -> &'static str {
            "FOO"
        }
    }
    assert_eq!(Foo::id(), "FOO");
    let f = Foo { a: 1 };
    f.bark();

    type Bar = Foo;
    // assert_eq!(!Bar::id(), "FOO");
    // error: unresolved name `Bar::id`
    // tests/lang.rs:628     assert_eq!(!Bar::id(), "FOO");
    let b = Bar { a: 2 };
    b.bark(); // this works though
    
    // impl Bar {
    //     // This would add a similarly named implementation, that is difficult to call
    //     // due to ambiguity.
    //     // Interestingly, it also affects Foo, as well as Bar !!
    //     // fn bark(&self) {
    //     //     println!("Grrrr {}", self.a);
    //     // }

    //     fn id() -> &'static str {
    //         "BAR"
    //     }   
    // }
    // b.bark(); // or f.bark();
    // error: multiple applicable methods in scope [E0034]
    // tests/lang.rs:625     f.bark();
    //                         ^~~~~~
    // tests/lang.rs:615:9: 617:10 note: candidate #1 is defined in an impl for the type `type_aliases::Foo`
    // tests/lang.rs:615         fn bark(&self) {
    // tests/lang.rs:616             println!("Wuff {}", self.a);
    // tests/lang.rs:617         }
    // tests/lang.rs:637:9: 639:10 note: candidate #2 is defined in an impl for the type `type_aliases::Foo`
    // tests/lang.rs:637         fn bark(&self) {
    // tests/lang.rs:638             println!("Grrrr {}", self.a);
    // assert_eq!(Bar::id(), "BAR");
}

#[test]
fn eq_on_float() {
    #[derive(PartialEq,Debug)]
    // #[derive(PartialEq,Eq,Debug)] // doesn't work, as there is no Eq for Floats
    struct PartialEqF32 {
        a: f32,
    }
    assert_eq!(PartialEqF32{a:1.0}, PartialEqF32{a:1.0});

    type MyFloat = f32;

    // This has nothing to do with the Eq problem encountered in rust-tracer.
    #[derive(PartialEq,Debug)]
    struct PartialEqMyFloat {
        a: MyFloat,
    }
    assert_eq!(PartialEqMyFloat{a:1.0}, PartialEqMyFloat{a:1.0});
}


#[test]
fn traversing_tuples() {

    // one day, I might understand the destructuring syntax ... !
    // The ampersand is required, and I'd like to be able to explain why.
    // see http://stackoverflow.com/questions/28405400/why-is-needed-to-destructure-a-list-of-tuples-during-iteration for an answer.git 
    for &(a, b, c) in [("hello", 1.0, 5), ("world", 2.0, 2)].iter() {
        println!("{} {} {}", a, b, c);
    }

    for a in &[1, 2, 3] {
        println!("{}", a);
    }

    for &a in [1, 2, 3].iter() {
        println!("{}", a);
    }
}

#[test]
fn into_iter() {
    use std::iter::IntoIterator;

    fn iterate<I: IntoIterator<Item=String>>(v: I) {
    }

    // iterate(&["foo".to_string()])
    // error: type mismatch resolving `<&[collections::string::String; 1] as core::iter::IntoIterator>::Item == collections::string::String`:
    // expected &-ptr,
    // found struct `collections::string::String` [E0271]
    
    // iterate(["foo".to_string()].iter())
    // type mismatch resolving `<core::slice::Iter<'_, collections::string::String> as core::iter::IntoIterator>::Item == collections::string::String`:
    // expected &-ptr,
    // found struct `collections::string::String` [E0271]

    // This works !
    iterate(vec!["foo".to_string()]);
}

// Doesn't compile anymore:
// tests/lang.rs:763:47: 763:60 error: type `[S]` does not implement any method in scope named `connect`
// tests/lang.rs:763         s.into_iter().collect::<Vec<S>>()[..].connect(", ")
// use std::slice::SliceConcatExt;
// fn generic_collect() {
//     use std::iter::IntoIterator;

//     fn connected<S, I>(s: I) -> String
//     where S: Str+Clone,
//           I: IntoIterator<Item=S> {
//         // have
//         s.into_iter().collect::<Vec<S>>()[..].connect(", ")
        
//         // want
//         // s.into_iter().connect(", ")
//         // error: type `<I as core::iter::IntoIterator>::IntoIter` does not implement any method in scope named `connect`
//         // tests/lang.rs:790         s.into_iter().connect(", ")
//     }

//     connected(&["foo", "bar"]);
// }

#[test]
fn pair_iterator_pattern() {
    fn pair_transformer<'a, I>(pairs: I) -> String
        where I: Iterator<Item=&'a (&'a str, &'a str)> {
        for pair in pairs {
            let &(a, b) = pair;
            // do something
        }
        String::new()
    }

    fn consuming_pair_transformer<'a, I>(pairs: I) -> String
        where I: Iterator<Item=(&'a str, &'a str)> {
        for pair in pairs {
            let (a, b) = pair;
            // do something
        }
        String::new()
    }

    // This works, natively
    pair_transformer([("a", "b")].iter());

    // This one too
    pair_transformer(vec![("a", "b")].iter());

    struct Pair<A: Copy, B: Copy> {
        first: A,
        second: B
    }

    impl<A: Copy, B: Copy> Pair<A, B> {
        fn from_tuple(t: (A, B)) -> Pair<A, B> {
            Pair {
                first: t.0,
                second: t.1
            }
        }

        fn as_tuple(&self) -> (A, B) {
            (self.first, self.second)
        }
    }

    // Consuming custom type instances
    let pairs = [Pair::from_tuple(("a", "b"))];
    // pair_transformer(pairs.iter().map(|p| &p.as_tuple()));
        //     tests/lang.rs:845:44: 845:56 error: borrowed value does not live long enough
        // tests/lang.rs:845     pair_transformer(pairs.iter().map(|p| &p.as_tuple()));
        //                                                              ^~~~~~~~~~~~
        // tests/lang.rs:845:22: 845:57 note: reference must be valid for the method call at 845:21...
        // tests/lang.rs:845     pair_transformer(pairs.iter().map(|p| &p.as_tuple()));
        //                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // tests/lang.rs:845:43: 845:56 note: ...but borrowed value is only valid for the block at 845:42
        // tests/lang.rs:845     pair_transformer(pairs.iter().map(|p| &p.as_tuple()));
        //                                                             ^~~~~~~~~~~~~
    // This works now ... 
    consuming_pair_transformer(pairs.iter().map(|p| p.as_tuple()));
}

#[test]
fn pair_iterator_pattern_borrowed() {
    use std::borrow::Borrow;

    fn pair_transformer<'a, I, T>(pairs: I) -> String
        where   T: Borrow<(&'a str, &'a str)>,
                I: Iterator<Item=T> {
        let mut s = String::new();
        for pair in pairs {
            let &(a, b) = pair.borrow();
            // do something
            s = s + a + b;
        }
        s
    }

    let pairs = [("a", "b")];
    assert_eq!(pair_transformer(pairs.iter()), "ab");
    assert_eq!(pair_transformer(pairs.iter().map(|p|(p.0, p.1))), "ab");
}

#[test]
fn pair_trait_for_iteration() {
    use std::borrow::Borrow;

    trait Pair<'a, A: ?Sized, B: ?Sized> {
        fn first_ref(&'a self) -> &'a A;
        fn second_ref(&'a self) -> &'a B;
    }

    struct PairOwned<A, B> {
        first: A,
        second: B,
    }

    // Only implemented for the cases we are interested in ...
    impl<'a, ARef: ?Sized, BRef: ?Sized, A, B> Pair<'a, ARef, BRef> for PairOwned<A,B>
        where A:    Borrow<ARef>,
              B:    Borrow<BRef> {
        fn first_ref(&'a self) -> &'a ARef {
            self.first.borrow()
        }
        fn second_ref(&'a self) -> &'a BRef {
            self.second.borrow()
        }
    }

    // It should also be possible to be more generic here with Borrow
    // But I wanted to leave your original implementation
    impl<'a, A: ?Sized, B: ?Sized> Pair<'a, A, B> for (&'a A, &'a B) {
        fn first_ref(&'a self) -> &'a A {
            self.0
        }
        fn second_ref(&'a self) -> &'a B {
            self.1
        }
    }

    fn pair_transformer<'a, I, T>(pairs: I) -> String
        where   T: Pair<'a, str, str> + 'a,
                I: Iterator<Item=&'a T> {
        let mut s = String::new();
        for pair in pairs {
            s = s
                + pair.first_ref().as_ref()
                + pair.second_ref().as_ref();
        }
        s
    }

    pair_transformer([PairOwned { first: "a".to_string(), second: "b".to_string() }].iter());
    pair_transformer([PairOwned { first: "a".to_string(), second: "b" }].iter()); // It is even possible to mix String and &str
    pair_transformer([PairOwned { first: "a", second: "b" }].iter());
    pair_transformer([("a", "b")].iter());
    pair_transformer(vec![("a", "b")].iter());
    pair_transformer(vec![("a", "b")][..].iter());
}

#[test]
fn serialize_json() {
    use rustc_serialize::json;

    // Automatically generate `Decodable` and `Encodable` trait implementations
    #[derive(RustcDecodable, RustcEncodable)]
    struct TestStruct  {
        data_int: u8,
        data_str: String,
        data_vector: Vec<u8>,
    }

    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };

    // Serialize using `json::encode`
    let encoded = json::encode(&object).unwrap();

    // Deserialize using `json::decode`
    let decoded: TestStruct = json::decode(encoded.as_ref()).unwrap();
}

#[test]
fn lifetime_never_used_bug() {
    // https://github.com/rust-lang/rust/issues/22798
    // CLOSED
    struct Foo<'a> {
        a: &'a str,
    }

    // struct Bar<'a, T = Foo<'a>> {
    //     b: T,
    // }

    use std::marker::PhantomData;
    struct BarWorking <'a, T = Foo<'a>> 
        where T: 'a {
        b: T,
        _m: &'a PhantomData<T>,
    }
}

#[test]
fn traits_and_generics() {
    // https://github.com/rust-lang/rust/issues/22834
    // CLOSED (Invalid)
    use std::default::Default;

    trait Maker {
        type Item;

        fn make(&mut self) -> Self::Item;
    }

    struct Foo<T> {
        a: T,
    }

    // struct Bar;
    // 
    // Fails
    // impl<T> Maker for Bar
    //     where T: Default  {
    //     type Item = Foo<T>;

    //     fn make(&mut self) -> Foo<T> {
    //         Foo {
    //             a: <T as Default>::default(),
    //         }
    //     }
    // }

    // Works
    use std::marker::PhantomData;
    struct Bar<T> {
        _m: PhantomData<T>
    }

    impl<T> Maker for Bar<T>
        where T: Default  {
        type Item = Foo<T>;

        fn make(&mut self) -> Foo<T> {
            Foo {
                a: <T as Default>::default(),
            }
        }
    }
}

#[test]
fn passing_static_refs() {
    trait Noop: Send {
        fn noop(&self){}
    }

    struct Foo {
        a: &'static str,
        b: &'static Noop,
    }

    impl Clone for Foo {
        fn clone(&self) -> Foo {
            Foo {
                a: self.a,
                b: self.b,
            }
        }
    }

}

#[test]
fn chrono_time_conversion() {
    // http://stackoverflow.com/questions/28747694/how-do-i-convert-a-chrono-datetimeutc-instance-to-datetimelocal
    use chrono::{Local, TimeZone};

    let utc = chrono::UTC::now();
    let loc = chrono::Local::now();

    println!("{:?}", utc);
    println!("{:?}", loc);
    println!("{:?}", utc.with_timezone(&Local));
    println!("{:?}", Local.from_utc_datetime(&utc.naive_local()));

}

#[test]
fn refcell_and_borrow_mut() {
    use std::cell::RefCell;
    use std::borrow::BorrowMut;

    struct Hub<C> {
        client: RefCell<C>
    }

    impl<C> Hub<C> 
        where C: BorrowMut<Client> {

        fn new(client: C) -> Hub<C> {
            Hub {
                client: RefCell::new(client)
            }
        }

        fn builder<'a>(&'a self) -> Builder<'a, C> {
            Builder {
                hub: self
            }
        }
    }

    struct Builder<'a, C: 'a> {
        hub: &'a Hub<C>
    }

    impl<'a, C> Builder<'a, C>
        where C: BorrowMut<Client> {
        fn use_client(self) {
            // 1: borrow_mut() of RefCell
            // 2: borrow_mut() of BorrowMut()
            // but doesn't work, as RefMut returned by 1) always yields RefCell
            loop {
                (*self.hub.client.borrow_mut()).borrow_mut().doit();
                break;
            }
        }
    }

    struct Client;
    impl Client {
        fn doit(&mut self) {
            println!("DID IT!!")
        }
    }


    // HUB USAGE
    {
        let h = Hub::new(Client);
        h.builder().use_client();
    }

    {
        let mut c = Client;
        let h = Hub::new(&mut c);
        h.builder().use_client();
    }
}

#[test]
fn borrow_oddity() {
    /// A utility type to perform a resumable upload from start to end.
    struct Foo<'a> {
        a: &'a u64,
    }

    impl<'a> Foo<'a> {
        fn mutate_internal(&mut self) {}
        
        fn mutate(&mut self) {
            self.mutate_internal();
            self.mutate_internal();
        }
    }
}

// #[test]
// http://stackoverflow.com/questions/28136739/variable-member-array-sizes-in-generic-types
// fn test_generic_arrays() {

//     struct Vec<T: Sized, Count> {
//         a: [T; Count]
//     }
// }

#[test]
fn boxed_write_and_mutability() {
    use std::io::{self, Write};
    fn make_stream() -> Box<Write> {
        Box::new(io::stdout())   
    }
    fn use_stream(stream: &mut Write) {
        stream.write("Hello\n".as_bytes()).ok();
    }

    use_stream(&mut make_stream());
}
