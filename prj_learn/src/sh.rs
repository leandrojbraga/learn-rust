#![allow(dead_code)]

use std::mem;

struct Point {
	x:f64,
	y:f64
}

fn origin() -> Point{
	Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {
	let p1 = origin(); // Stack allocated variables
	let p2 = Box::new(origin()); // Heap allocated rectangle

	println!("p1 takes up {} bytes", mem::size_of_val(&p1));
	println!("p2 takes up {} bytes", mem::size_of_val(&p2));
	println!("p2.x = {}", p2.x);
	
	let p3 = *p2; // unboxing
	println!("p3 takes up {} bytes", mem::size_of_val(&p3));
	println!("p3.x = {}", p3.x);
}