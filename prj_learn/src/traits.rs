#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


trait Animal {
	fn create(name: &'static str) -> Self;

	fn name(&self) -> &'static str; // Não implementado, portanto é obrigatória uma implementação de método no "impl"

	fn talk(&self) { // Implementado, portanto não é obrigatória uma implementação de método no "impl"
		println!("{} cannot talk", self.name());
	}
}

struct Human {
	name: &'static str
}

impl Animal for Human {
	fn create(name: &'static str) -> Human {
		Human{ name: name }
	}

	fn name(&self) -> &'static str {
		self.name // name da struct Human
	}

	fn talk(&self) {
		println!("{} say hello", self.name());
	}
}

struct Giraffe {
	name: &'static str
}

impl Animal for Giraffe {
	fn create(name: &'static str) -> Giraffe {
		Giraffe{ name: name }
	}

	fn name(&self) -> &'static str {
		self.name // name da struct Giraffe
	}
}

trait Summable<T> {
	fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
	fn sum(&self) -> i32 {
		self.iter().fold(0, |sum, x| sum + x)
	}
}

pub fn traits() {
	//let h = Human{ name: "Leandro" };
	let h = Human::create("Leandro");
	h.talk();

	//let g = Giraffe{ name: "Gigi" };
	let g:Giraffe = Animal::create("Gigi");
	g.talk();

	let v = vec![1,2,3];
	println!("sum = {}", v.sum()); // error if not implement Summable: the method `sum` exists for struct `Vec<{integer}>`, but its trait bounds were not satisfied
}

use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
	radius: f64,
}

#[derive(Debug)]
struct Square {
	side: f64,
}

trait Shape {
	fn area(&self) -> f64;
}

impl Shape for Square {
	fn area(&self) -> f64 {
		self.side * self.side
	}
}

impl Shape for Circle {
	fn area(&self) -> f64 {
		self.radius * self.radius * std::f64::consts::PI
	}
}

//fn print_info(shape: impl Shape + Debug) {
//fn print_info<T: Shape + Debug>(shape: T) {
fn print_info<T>(shape: T)
  where T: Shape + Debug {
  	
	println!("{:?}", shape);
	println!("The area is {}", shape.area());
}

pub fn trait_parameters() {
	let c = Circle{ radius: 2.0 };
	print_info(c);
}
