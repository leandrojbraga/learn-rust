#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod sh;

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

    sh::stack_and_heap();

}
