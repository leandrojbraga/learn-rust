#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(bare_trait_objects)]

trait Printable {
	fn format(&self) -> String;
}

impl Printable for i32 {
	fn format(&self) -> String {
		format!("i32: {}", *self)
	}
}

impl Printable for String {
	fn format(&self) -> String {
		format!("string: {}", *self)
	}
}

fn print_it_static<T: Printable>(z: T) {
	println!("{}", z.format());
} // monomorphisation

pub fn static_dispatch() {
	let n = 123;
	let l = "Hello".to_string();

	//println!("{}", n.format());
	//println!("{}", l.format());

	print_it_static(n);
	print_it_static(l);
	
	
}

// more expensive call than print_it_static
// fn print_it_dynamic(z: &Printable) {
// 	println!("{}", z.format());
// }

struct Circle { radius: f64 }
struct Square { side: f64 }

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



pub fn dynamic_dispatch() {
	// let n = 123;
	// let l = "Hello".to_string();

	// print_it_dynamic(&n);
	// print_it_dynamic(&l);

	let shapes:[&Shape; 4] = [
		&Circle{ radius:1.0 },
		&Square{ side: 3.0 },
		&Circle{ radius:2.0 },
		&Square{ side: 4.0 }
	];

	for (i, shape) in shapes.iter().enumerate() {
		println!("Shape #{} has area {}", i, shape.area());
	}

}