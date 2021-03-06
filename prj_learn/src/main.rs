#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

mod sh;
mod pm;
mod structures;
mod functions;
mod traits;
mod drop;
mod operator_overloading;
mod dispatch;
mod vectors;
mod circular_references;

use std::mem;

//const: An unchangeable value (the common case).
const MEANING_OF_LIFE:u8 = 45; // no fixed address

//static: A possibly mutable variable with 'static lifetime. 
//		  The static lifetime is inferred and does not have to be specified. 
//		  Accessing or modifying a mutable static variable is "unsafe".
static mut Z:i32 = 123;


fn core_data_types() {
	// ------------------------------------------------------------------
	// ---------------------------Integers-------------------------------
	// ------------------------------------------------------------------
	// u = unsigned, "0" to "2^N - 1"
	//     u8, u16, u32, u64, u128
	// ------------------------------------------------------------------
	// i = signed, "-2^(N-1)" to "2^(N-1) - 1"
	//     i8, i16, i32, i64, i128
	// ------------------------------------------------------------------
	// usize, isize
	//     depend on the kind of computer your program is running on:
	//     64 bits if you’re on a 64-bit architecture and 
	//     32 bits if you’re on a 32-bit architecture
	// ------------------------------------------------------------------
	let number1: u8 = 123; // 8 bits, [0, 255], immutable
	println!("number1 = {}", number1); 

	let mut number2: i8 = 0; // 8 bits, [-128, 127], mutable
	println!("number2 = {} before", number2);
	number2 = 33;
	println!("number2 = {} after", number2);

	let mut number3 = 123456789; // i32 = 32 bits = 4 bytes
	println!("number3 = {}, takes up {} bytes", 
		number3, mem::size_of_val(&number3) // &number3 = pointer of variable
	);
	number3 = -1;
	println!("number3 = {}", number3);

	let n: isize = 123;
	let size_of_n: usize = mem::size_of_val(&n);
	println!(
		"n = {}, takes up {} bytes, {}-bits OS",
		n, size_of_n, size_of_n*8
	);


	// ------------------------------------------------------------------
	// ---------------------------Charactes------------------------------
	// ------------------------------------------------------------------
	let letter: char = 'x'; // specified with single quotes
	println!(
		"{} is a char, size = {} bytes", 
		letter, mem::size_of_val(&letter)
	);


	// ------------------------------------------------------------------
	// -------------------------Floating-point---------------------------
	// ------------------------------------------------------------------
	// f32 = 32 bits
	// f64 = 64 bits (default because on modern CPUs)
	// Floating-point numbers are represented according to the IEEE-754 standard.
	// The f32 type is a single-precision float, and f64 has double precision.
	// ------------------------------------------------------------------
	let price1: f32 = 2.5;
	println!(
		"price1 = {}, size = {} bytes", 
		price1, mem::size_of_val(&price1)
	);

	let price2 = 5.99;
	println!(
		"price2 = {}, size = {} bytes", 
		price2, mem::size_of_val(&price2)
	);

	// ------------------------------------------------------------------
	// -----------------------------Boolean------------------------------
	// ------------------------------------------------------------------
	let verdade: bool = true; // false
	println!(
		"verdade = {}, size = {} bytes", 
		verdade, mem::size_of_val(&verdade)
	);
}

fn operators() {
	// ------------------------------------------------------------------
	// ----------------------------Arithmetic----------------------------
	// ------------------------------------------------------------------
	// addition: "+"
	// subtraction: "-"
	// multiplication: "*"
	// division: "/"
	// remainder: "%"  
	// ------------------------------------------------------------------
	let mut result = 2+3*4;
	println!("{}", result);
	result = result + 1;
	result -= 2; // +=, -=, *=, /=, %=
	println!("remainder of {} / {} = {}",result, 3, (result%3));

	// ------------------------------------------------------------------
	// -------------------------------Power------------------------------
	// ------------------------------------------------------------------
	// ------------------------------------------------------------------
	// i32::pow  => power interger to interger
	// f64::powi => power with float to interger
	// f64::powf => power with float to float
	// ------------------------------------------------------------------
	let result_cubed = i32::pow(result, 3);
	println!("{} cubed is {}", result, result_cubed);

	let float_result = 2.5;
	let float_result_cubed = f64::powi(float_result, 3);
	let float_result_to_pi = f64::powf(float_result, std::f64::consts::PI);
	println!("{fr} cubed {frc}, {fr}^pi = {frpi}", fr=float_result, frc=float_result_cubed, frpi=float_result_to_pi);


	// ------------------------------------------------------------------
	// -------------------------------Bitwise----------------------------
	// ------------------------------------------------------------------
	// Just with intergers
	// OR: "|"
	// AND: "&"
	// XOR: "^"
	// NOR: "!"
	// ------------------------------------------------------------------
	let b = 1 | 2;
	println!("1|2 = {}", b);
	// 01 = 1_10
	// OR
	// 10 = 2_10
	// 11 = 3_10
	let two_to_10 = 1 << 10;
	println!("2^10 = {}",two_to_10);
	// 00000000001 = 1_10
	// << = desloca
	// 10000000000 = 1024_10

	// ------------------------------------------------------------------
	// -------------------------------Logical----------------------------
	// ------------------------------------------------------------------
	// Equal: "=="
	// Greater than: ">"
	// Greater than or equal: ">="
	// Less than: "<"
	// Less than or equal: "<=" 
	// ------------------------------------------------------------------
	let pi_less_4 = std::f64::consts::PI < 4.0; // true
	let x = 5;
	let x_is_5 = x == 5; // true
}


fn scope_and_shadowing() {
	let a = 123;
	//shadowing
	let a = 3210;

	//scope
	{
		let b = 456;
		println!("scope inside, b = {}", b);

		let a = 987;
		println!("scope inside, a = {}", a);
	}

	println!("scope outside, a = {}", a);

	// println!("scope outside, b = {}", b); // not working
}


fn if_statement(){
	let temp = 35;

	if temp > 30 {
		println!("really hot outside");
	}
	else if temp < 10 {
		println!("really cold!");
	}
	else {
		println!("temperature is OK");
	}

	let today = if temp > 20 {"sunny"} else {"cloudy"};
	println!("today is {}", today);

	println!("it is {}", 
		if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"}
	);

	println!("it is {}", 
		if temp > 20 {
			if temp > 30 {"very hot"} else {"hot"}
		} else if temp < 10 {"cold"} else {"OK"}
	);
}


fn while_and_loop() {
	let mut x = 1;

	while x < 1000 {
		x *= 2;

		if x == 64 { continue; }

		println!("x = {}", x);
	}

	let mut y = 1;

	loop { // while true
		y *= 2;

		println!("y = {}", y);

		if y == 1<<10 { break; }	 

	}
}

fn for_loop() {

	for x in 1..11 { // [1,2,3,4,5,6,7,8,9,10]
		if x == 3 { continue; }

		if x == 8 { break; }

		println!("x = {}", x);
	}

	print!("reverse count ");
	for y in (1..4).rev() { // [3,2,1]
		print!("{}...", y);
	} 
	println!("0");

	for (i,z) in (30..41).enumerate() {
		println!("{}: {}", i, z);
	}

	let numbers = 1..=5;

	for n in numbers {
        println!("the value is: {}", n);
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

	
}


fn match_country(country_code:u32) -> String {
	match country_code {
		44 => "UK",
		45 => "BR",
		46 => "Sweden",
		7 => "Russia",
		1..=1000  => "unknown", // value between 1..1000 (except the list up)
		_ => "invalid" // other value, is the default return. if comment: non-exhaustive patterns: `0_u32`
	}.to_string()
}

fn match_statement() {
	let mut country_code:u32 = 45;
	println!("The country with code {} is {}", country_code, match_country(country_code));

	country_code = 7;
	println!("The country with code {} is {}", country_code, match_country(country_code));

	country_code = 10;
	println!("The country with code {} is {}", country_code, match_country(country_code));

	country_code = 1001;
	println!("The country with code {} is {}", country_code, match_country(country_code));

	let truth = true;
	let result = match truth {
		true => "truth",
		false => "lie" // if comment: non-exhaustive patterns: `false` not covered
		//to match bool type is not necessary a default return
	};
	println!("It is {}",result);

}

use std::io::stdin;

enum State {
	Locked,
	Failed,
	Unlocked
}

fn combination_lock() {
	let pass = String::from("1234");
	let mut tries:u8 = 0;
	let limit:u8 = 3;
	let mut state = State::Locked;
	let mut entry = String::new();


	loop {
		match state {
			State::Locked => {
				let mut input = String::new();
				tries += 1;
				println!("Password: ");
				match stdin().read_line(&mut input) {
					Ok(_) => {
						entry.push_str(&input.trim_end());
					}
					Err(_) => continue
				}
				if entry == pass {
					state = State::Unlocked;
					continue;
				}
				else {
					state = State::Failed;
				}
			}
			State::Failed => {
				print!("FAILED! ");
				if (limit - tries) == 0 {
					println!("Account blocked. Try again in a few minutes.");
					return;
				}
				else {
					println!("Try again. You have {} more attempts", (limit - tries));
					entry.clear(); // entry = "";
					state = State::Locked;
					continue;
				}
			}
			State::Unlocked => {
				println!("UNLOCKED!!!");
				println!("Welcome to Rust!");
				return;
			}
		}
	}
}


enum Color {
	Red,
	Green,
	Blue,
	RgbColor(u8, u8, u8), // tuple
	CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, //struct
}

fn match_color(c: Color) {
	match c {
		Color::Red => println!("R"),
		Color::Green => println!("G"),
		Color::Blue => println!("B"),
		Color::RgbColor(0,0,0)
		| Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} => println!("Black"), // only is black:255
		Color::CmykColor{yellow:255,..} => println!("Yellow"), // only is yellow:255
		Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
		_ => ()
	}
}

fn enumerates() {
	let mut c:Color = Color::Red;
	match_color(c);

	c = Color::RgbColor(0,0,0);
	match_color(c);

	c = Color::RgbColor(1,54,176);
	match_color(c);

	c = Color::CmykColor{cyan:1, magenta:45, yellow:34, black:23};
	match_color(c);

	c = Color::CmykColor{cyan:1, magenta:45, yellow:34, black:255};
	match_color(c);

	c = Color::CmykColor{cyan:1, magenta:45, yellow:255, black:25};
	match_color(c);

}

fn option_t_and_if_let_while_let() {
	let x = 3.0;
	let y = 2.0;

	//Option -> Some(v) | None
	let result =
		if y != 0.0 { Some(x/y) } else { None };

	match result {
		Some(z) => println!("{}/{}={}", x,y,z),
		None => println!("cannot divide by zero")
	}

	if let Some(z) = result {
		println!("result = {}", z);
	}
}

fn arrays(){
	let mut numbers:[i32;5] = [1,2,3,4,5];

	println!("numbers has {} elements, first is {}", numbers.len(), numbers[0]);

	numbers[0] = 9;
	println!("numbers[0] = {}", numbers[0]);

	//print debug
	println!("{:?}", numbers);

	if numbers != [1,2,3,4,5] {
		println!("does not match");
	}

	if numbers == [9,2,3,4,5] {
		println!("match");
	}

	//if numbers == [9,2,3,4,5, 6] { () } // error: can't compare `[i32; 5]` with `[i32; 6]` (different size elements)

	let b = [1; 10];
	println!("b = {:?}, took up {} bytes", b, mem::size_of_val(&b));

	let c = [1u8; 10];
	println!("c = {:?}, took up {} bytes", c, mem::size_of_val(&c));

	let mtx:[[f32;3];2] = 
	[
		[1.2, 3.1, 0.0],
		[3.2, 0.1, 1.0]
	];

	println!("{:?}", mtx);

	for r in 0..mtx.len() {
		for c in 0..mtx[r].len() {
			if r == c {
				println!("mtx[{}][{}] = {}", r, c, mtx[r][c]);
			}
		}
	}

}

fn slices() {
	let mut data = [1,2,3,4,5];

	//let slice:&[i32] = &data[1..4];
	//println!("slice first elem = {}, len = {}", slice[0], slice.len());
	//slice[0] = 87; // error: cannot assign to `slice[_]` which is behind a `&` reference
	//println!("slice {:?}", slice);
	//println!("data = {:?}", data);

	let slice: &mut[i32] = &mut data[1..4];
	println!("slice first elem = {}, len = {}", slice[0], slice.len());
	slice[0] = 87;
	println!("slice {:?}", slice);
	println!("data = {:?}", data);

}

fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
	(x+y, x*y)
}

fn tuples() {
	let x = 3;
	let y = 4;
	let sp = sum_and_product(x,y);

	println!("sp = {:?}", sp);
	println!("{0} + {1} = {2} | {0} + {1} = {3}", x, y, sp.0, sp.1);

	//destructuring
	let (a,b) = sp;
	println!("a = {}, b = {}", a,b);

	let sp2 = sum_and_product(4,7);
	let combined = (sp, sp2);
	println!("{:?}", combined);
	println!("last elem = {}", (combined.1).1);
	let ((c,d),(e,f)) = combined;

	let foo = (true, 42.9, -1i8);
	println!("{:?}", foo);

	let meaning = (42,); // tuple of single element
}

// Option<T>
#[derive(Debug)] // to print
struct Point<T> {
	x: T,
	y: T
}

struct Point2<T,V> {
	x: T,
	y: V
}

#[derive(Debug)] // to print
struct Line<T> {
	start: Point<T>,
	end: Point<T>
}

fn generics() {
	let p1 = Point{x:0, y:0};
	let p2 = Point{x:1.2, y:3.4};

	//explicit
	let p3 = Point::<u8>{x:4, y:7};
	let p4:Point<f64> = Point{x:3f64, y:2.8};

	let p5:Point2<i32,u16> = Point2{x:-10, y:30};
	let p6:Point2<f64,u8> = Point2{x:8.93, y:0};

	//let line = Line{start:p1, end:p2}; // error: mismatched types `p2` expected integer, found floating-point number 
	let line = Line{start:p1, end:p3};
	println!("{:#?}", line);

}



use std::collections::HashMap;

fn hashmaps() {
	let mut shapes = HashMap::new();

	shapes.insert(String::from("triangle"), 3);
	shapes.insert(String::from("square"), 4);

	println!("a square has {} sides", 
		shapes["square".into()]);

	shapes.insert("square".into(), 5);

	for (key, value) in &shapes {
		println!("{} : {}", key, value);
	}

	shapes.entry("cicle".into()).or_insert(1);
	println!("{:?}", shapes);
	{
		let actual = shapes.entry("cicle".into()).or_insert(2);
		*actual = 0;
	}
	println!("{:?}", shapes);	
}

use std::collections::HashSet;

fn hashsets() {
	let mut greeks = HashSet::new();
	greeks.insert("gamma");
	greeks.insert("delta");
	println!("{:?}", greeks);

	greeks.insert("delta");
	println!("{:?}", greeks);

	let added_vega = greeks.insert("vega");
	if added_vega {
		println!("We added vega!")
	}
	let added_vega_again = greeks.insert("vega");
	if !added_vega_again {
		println!("We don't added vega again!")
	}

	if !greeks.contains("kappa") {
		println!("We don't contains kappa");
	}

	let removed = greeks.remove("delta");
	if removed {
		println!("We removed delta");
	}
	println!("{:?}", greeks);

	let _1_5: HashSet<_> = (1..=5).collect();
	let _6_10: HashSet<_> = (6..=10).collect();
	let _1_10: HashSet<_> = (1..=10).collect();
	let _2_8: HashSet<_> = (2..=8).collect();

	// subset = if other contains at least all the values in self
	println!("subset - is {:?} a subset of {:?}? {}",
		_2_8, _1_10,
		_2_8.is_subset(&_1_10));
	
	// disjoint = if self has no elements in common with other
	println!("disjoint - is {:?} a disjoint of {:?}? {}",
		_1_5, _6_10,
		_1_5.is_disjoint(&_6_10));

	// union = return all the values in self or other, without duplicates
	println!("union - items in either {:?} and {:?} are {:?}",
		_2_8, _6_10,
		_2_8.union(&_6_10));

	// intersection = return values that are both in self and other
	println!("intersection - items in {:?} and {:?} are {:?}",
		_2_8, _6_10,
		_2_8.intersection(&_6_10));

	// difference = return the values that are in self but not in other.
	println!("difference - items in either {:?} but not in {:?} are {:?}",
		_2_8, _6_10,
		_2_8.difference(&_6_10));

	// symmetric_difference = return values that are in self or in other but not in both.
	println!("symmetric_difference - items in either {:?} or {:?} but not in both are {:?}",
		_2_8, _6_10,
		_2_8.symmetric_difference(&_6_10));

}

fn iterators() {
	let vec = vec![3,2,1];

	// for x in vec {
	// 	println!("{}", x);
	// }
	// println!("{:?}", vec); // error: value borrowed here after move

	for x in &vec {
		println!("{}", *x);
	}
	for x in vec.iter().rev() {
		println!("we got {}", x);
	}
	println!("vec = {:?}", vec);

	let mut vec2 = vec![3,2,1];
	for x in vec2.iter_mut() {
		*x += 2;
	}
	println!("vec2 = {:?}", vec2);

	let mut vec3 = vec![6];
	vec3.extend(vec2);
	println!("vec3 = {:?}", vec3);
	//println!("vec2 = {:?}", vec2); // error: value borrowed here after move

}

fn strings() {
	//utf-8
	let s:&'static str = "Hello there!"; // &str = string slice
	//s = "abc"
	//let h = s[0];

	for c in s.chars().rev() {
		println!("{}", c);
	}

	if let Some(first_char) = s.chars().nth(0) {
		println!("first letter is {}", first_char);
	}

	//heap
	//String 
	let mut letters = String::new(); // Datatype for string that can be modified
	let mut l = 'a' as u8;
	while l <= ('z' as u8) {
		letters.push(l as char);
		letters.push_str(";");
		l += 1;
	}
	println!("letters = {}", letters);

	//&str <> String
	let u:&str = &letters;
	println!("u = {}", u);

	//concatentation
	//String + str
	let z = letters + "0;1;2;3;4;5;6;7;8;9;";
	//println!("{}", letters); // error: value borrowed here after move
	println!("z = {}", z);

	let mut abc = "Hello world".to_string(); // or String::from("Hello world");
	abc.remove(0);
	abc.push_str("!!!");
	println!("{}", abc.replace("ello", "goodbye"));
}

fn string_formatting() {
	let name = "Leandro";
	let greeting = format!("hi, I'm {}, nice to meet you", name);
	println!("{}", greeting);

	let hello = "hello";
	let rust = "rust";
	let hello_rust = format!("{}, {}!", hello, rust);
	println!("{}", hello_rust);

	let run = "run";
	let forest = "forest";
	let run_forest = format!("{0}, {1}, {0}!", run, forest);
	println!("{}", run_forest);

	let the_name = format!("the name's {last}. {first} {last}.", 
		first = "James",
		last = "Bond");
	println!("{}", the_name);

	let mixed = format!("{1} {} {0} {} {data}", 
		"alpha",
		"beta",
		//"gamma", // error: argument never used
		data = "delta");
	println!("{}", mixed);
}

use rand::Rng;

fn number_guessing_game(){
	let number = rand::thread_rng().gen_range(1, 101);

	loop {
		println!("Enter your guess: ");
		let mut buffer  = String::new();

		match stdin().read_line(&mut buffer) {
			Ok(_) => {
				let parsed = buffer.trim_end().parse::<i64>();
				match parsed {
					Ok(guess) => {
						if guess < 1 || guess > 100 {
							println!("Your guess is out of range");
						} else if guess < number {
							println!("Your guess is too low");
						} else if guess > number {
							println!("Your guess is too high");
						} else {
							println!("Correct!");
							break
						}
					},
					Err(e) => {
						println!("Could not read your input. {}. Try again!", e);
					}
				}
			},
			Err(_) => continue,
		}
	}
}

struct Person {
	name: String
}

impl Person {
	// fn new(name: &str) -> Person {
	// 	Person { name: name.to_string() }
	// }

	//Convert param type where possible
	//fn new<S: Into<String>>(name: S) -> Person {
	fn new<S>(name: S) -> Person 
	  where S: Into<String> {
		Person { name: name.into() }
	}
}

fn into() {
	let john = Person::new("John");
	println!("{}", john.name);

	let name: String = "Jane".to_string();
	//let jane = Person::new(name.as_ref()); // new(name: &str)
	let jane = Person::new(name); // new<S: Into<String>>(name: S)
	println!("{}", jane.name);
}

fn ownership() {
	let v = vec![1,2,3];
	//let v2 = v;
	//println!("{:?}",v); // error: borrow of moved value: `v`

	//let foo = |v:Vec<i32>| ();
	//foo(v);
	//println!("{:?}",v); // error: borrow of moved value: `v`

	//let b1 = Box::new(1);
	//let b2 = b1;
	//println!("{:?}", *b1); // error: borrow of moved value: `b1`

	let n1 = 1; //i32 implement Copy
	let n2 = n1;
	println!("{}", n1);
	// others type implement Copy: 
	// All the integer types, such as u32.
	// The Boolean type, bool, with values true and false.
	// All the floating point types, such as f64.
	// The character type, char.
	// Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

	let print_vector = |x: Vec<i32>| -> Vec<i32> {
		println!("x = {:?}", x);
		x
	};

	let vv = print_vector(v);
	println!("vv = {:?}", vv);
}

fn borrowing() {
	let print_vector = |x: &Vec<i32>| {
		println!("x = {:?}", x);
	};

	let v = vec![1,2,3];
	print_vector(&v);
	println!("v = {:?}", v);

	let mut n1 = 40;
	{
		let n2 = &mut n1;
		*n2 += 2;
	}	
	println!("n1 = {}", n1);

	let mut z = vec![3,2,1];

	for i in &z {
		println!("i = {}", i);
		//z.push(5); // error: cannot borrow `z` as mutable because it is also borrowed as immutable
	}

}

struct Employee { name: String }
impl Employee {
	//fn get_name_ref(&self) -> &String {
	fn get_name_ref<'y>(&'y self) -> &'y String {
		&self.name
	}
}

struct Company<'z> {
	name: String,
	ceo: &'z Employee
}

fn lifetime() {
	let boss = Employee{ name: String::from("Elon Musk") };
	let tesla = Company{ name: String::from("Tesla"), ceo: &boss };
	// error if not implemet Company<'a>: missing lifetime specifier

	let mut name: &String;
	{
		let e = Employee{ name: String::from("John") };
		name = e.get_name_ref();
		println!("{}", name);
	}
}

struct Persona<'a> {
	name: &'a str
}

impl<'a> Persona<'a> {
	fn talk(&self) {
		println!("Hi, my name is {}.", self.name);
	}
}

fn lifetime_in_structure_implementation() {
	let p = Persona{ name: "Jane" };
	p.talk();
}

use std::rc::Rc;

struct Dog {
	name: Rc<String>
}

impl Dog {
	fn new(name: Rc<String>) -> Dog {
		Dog{ name: name }
	}

	fn greet(&self) {
		println!("Hi, my name is {}.", self.name);
	}
}

fn reference_counted_variables() {
	let mut name = Rc::new("Doguinho".to_string());
	println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
	{
		let dog = Dog::new(name.clone());
		println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
		dog.greet();
	}
	println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
}

use std::sync::{Mutex, Arc};
use std::thread;

struct Lhama {
	name: Arc<String>,
	state: Arc<Mutex<String>>

}

impl Lhama {
	fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Lhama {
		Lhama{ name: name, state: state }
	}

	fn greet(&self) {
		let mut state = self.state.lock().unwrap();
		state.clear();
		state.push_str("excited");

		println!("Hi, my name is {} and i'm {}.", self.name, state.as_str());
	}
}


fn atomic_reference_counted_variables_and_mutex() {
	let name = Arc::new("Carlos".to_string());
	let state = Arc::new(Mutex::new("bored".to_string()));
	let lhama = Lhama::new(name.clone(), state.clone());

	let t = thread::spawn(move || {
		lhama.greet();
	});
	println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());

	t.join().unwrap();
}

use std::time;

fn spawning_and_joining_threads() {
	let handle = thread::spawn(|| {
		for _ in 1..10 {
			print!("+");
			thread::sleep(time::Duration::from_millis(500));
		}
	});

	for _ in 1..10 {
		print!("*");
		thread::sleep(time::Duration::from_millis(300));
	}

	handle.join();

}

//extern crate phrases;

use phrases::greetings::{english, french, portuguese};

fn building_modules_cand_crates() {
	println!("English: {}, {}",
		english::hello(),
		english::goodbye());
	println!("French: {}, {}",
		french::hello(),
		french::goodbye());
	println!("English: {}, {}",
		portuguese::hello(),
		portuguese::goodbye());
	
}

fn main() {
    //core_data_types();
    
    //operators();
    
    //scope_and_shadowing();
    
    //println!("MEANING_OF_LIFE = {}", MEANING_OF_LIFE);
    //unsafe
    //{
    //	println!("Z = {}", Z);
    //	Z = 456;
    //	println!("new Z = {}", Z);
    //}

    //sh::stack_and_heap();

    //if_statement();

    //while_and_loop();

    //for_loop();

    //match_statement();

    //combination_lock();

    //structures::structures();

    //enumerates();

    //option_t_and_if_let_while_let();

    //arrays();

    //slices();

    //tuples();

    //pm::pattern_matching();

    //generics();

    //vectors::vectors();

    //hashmaps();

    //hashsets();

    //iterators();

    //strings();

    //string_formatting();

    number_guessing_game();

    //functions::functions();

    //structures::methods();

    //functions::closures();

    //functions::higher_order_functions();

    //traits::traits();

    //traits::trait_parameters();

    //into();

    //drop::drops();

    //operator_overloading::operator_overloading();

    //dispatch::static_dispatch();

    //dispatch::dynamic_dispatch();

    //vectors::vectors_of_different_objects();

    //ownership();

    //borrowing();

    //lifetime();

    //lifetime_in_structure_implementation();

    //reference_counted_variables();

    //atomic_reference_counted_variables_and_mutex();

    //circular_references::circular_references();

    //spawning_and_joining_threads();

    //building_modules_cand_crates();

}
