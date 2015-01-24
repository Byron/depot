#![allow(unstable)]

extern crate test;

const ARRAY_SIZE: usize = 1<<16;

#[bench]
fn iter_find_overhead(b: &mut test::Bencher) {
    let a = [1i32; ARRAY_SIZE];

    // BAD
    b.iter(|| {
        match a.iter().find(|&&x| x > 1i32) {
            None => (),
            _ => assert!(false, "Expecting only 1 in there"),
        }
    });
}


#[bench]
fn static_array_aggregation_with_index_access(b: &mut test::Bencher) {
    let a = [1i32; ARRAY_SIZE];

    // BAD
    b.iter(|| {
        let mut res = 0us;
        for i in range(0, a.len()) {
            res += a[i] as usize;
        }
        assert!(a.len() == res);
    });
}

#[bench]
fn static_array_aggregation_with_iter(b: &mut test::Bencher) {
    let a = [1i32; ARRAY_SIZE];

    // GOD
    b.iter(|| {
        let mut res = 0us;
        for &i in a.iter() {
            res += i as usize;
        }
        assert!(a.len() == res);
    });
}

#[bench]
fn static_array_aggregation_with_fold(b: &mut test::Bencher) {
    let a = [1i32; ARRAY_SIZE];

    // GOD
    b.iter(|| {
        assert!(a.len() == a.iter().fold(0us, |a, &b| a + b as usize));
    });
}

#[bench]
fn vector_aggregation_with_iter(b: &mut test::Bencher) {
    let mut a = Vec::new();
    a.resize(ARRAY_SIZE, 1i32);

    let mut total = 0us;
    b.iter(|| {
        let mut res = 0us;
        for &i in a.iter() {
            res += i as usize;
        }
        assert!(a.len() == res);
        total += res;
    });
}