#![allow(unused_assignments)]

fn print_value(x: i32) {
	println!("value = {}", x);
}

fn increase(x: &mut i32) {
	*x += 1;
}

// function_name(param: type) -> return type
fn product(x: i32, y: i32) -> i32 {
	x * y // return, última expressão sem ";" da função
}

pub fn functions() {
	print_value(33);

	let mut z = 1;
	increase(&mut z);
	println!("z = {}", z);

	let a = 3;
	let b = 5;
	let p = product(a, b);
	println!("{} * {} = {}", a, b, p);

}

fn say_hello() { println!("Hello"); }

pub fn closures() {
	let sh = say_hello;
	sh();

	let plus_one = |x:i32| -> i32 { x + 1 };
	let a = 6;
	println!("{} + 1 = {}", a, plus_one(a));

	let mut two = 2;
	{
		let plus_two = |x| {
			let mut z = x;
			z += two;
			z
		};
		println!("{} + 2 = {}", 3, plus_two(3));
	}

	let borrow_two = &mut two;

	// T: by value
	// T&
	// &mut &
	let plus_three = |x:&mut i32| *x += 3;
	let mut f = 12;
	plus_three(&mut f);
	println!("f = {}", f);

	let plus_four = |mut x:i32| x += 4;
	let mut g = 16;
	plus_four(g);
	println!("g = {}", g);
}

// functions that take functions
// f(g) { let x = g(); }

// functions that return functions
// generators
// f() -> g

fn is_even(x: u32) -> bool {
	x % 2 == 0
}

fn greater_than(limit: u32)
	-> impl Fn(u32) -> bool
{
	move |y| y > limit
}

pub fn higher_order_functions(){
	// Sum of all even square < 500

	let limit = 500;
	let mut sum = 0;

	//let above_limit = |y| y > limit;
	let above_limit = greater_than(limit);

	for i in 0.. {
		let isq = i*i;

		//if isq > limit {
		if above_limit(isq) {
			break;	
		}
		else if is_even(isq) {
			sum += isq;
		}
	}
	println!("loop sum = {}", sum);

	let sum2 = (0..)
		.map(|x| x*x)
		.take_while(|&x| x < limit)
		.filter(|x:&u32| is_even(*x))
		.fold(0, |sum, x| sum + x);

	println!("hof sum = {}", sum2);
}