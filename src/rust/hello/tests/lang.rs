
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
	let s = "Hi";   				// This is a string slice
	assert!(s.len() == 2);

	let mut si = s.to_string();
	assert!(si.len() == s.len());
}		