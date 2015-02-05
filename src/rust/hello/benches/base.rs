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

#[bench]
fn static_array_aggregation_with_fold_on_optionals(b: &mut test::Bencher) {
    let mut a = Vec::new();
    a.resize(ARRAY_SIZE, Some(1i32));

    // GOD
    b.iter(|| {
        assert!(a.len() == a.iter().fold(0us, |a, &b| a + b.unwrap() as usize));
    });
}


#[bench]
fn static_array_aggregation_with_fold(b: &mut test::Bencher) {
    let a = [1i32; ARRAY_SIZE];

    // GOD
    b.iter(|| {
        assert!(a.len() == a.iter().fold(0us, |a, &b| a + b as usize));
    });
    b.bytes += ARRAY_SIZE as u64;
}

#[bench]
fn static_array_aggregation_with_fold_on_results(b: &mut test::Bencher) {
    // Just fill in what's not inferable !
    let init: Result<_, &str> = Ok(1i32);
    let mut a = Vec::new();
    a.resize(ARRAY_SIZE, init);

    // GOD
    b.iter(|| {
        assert!(a.len() == a.iter().fold(0us, |a, &b| a + b.unwrap() as usize));
    });
}


#[bench]
fn array_index_checking(b: &mut test::Bencher) {
    const ASIZE: usize = 4096;
    let mut a = [2u8; ASIZE];
    b.iter(|| {
        for i in range(1, ASIZE-1) {
            a[i] = a[i-1] * a[i+1];
        }
    })
}


