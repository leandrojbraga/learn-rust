#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(bare_trait_objects)]



pub fn vectors() {
	let mut vetor = Vec::new();

	vetor.push(1); // add element
	vetor.push(2);
	vetor.push(3);

	println!("{:?}", vetor);

	println!("vetor[0] = {}", vetor[0]);

	//let idx:i32 = 0; // error: the type `[{integer}]` cannot be indexed by `i32`
	let mut idx:usize = 0;
	vetor[idx] = 4;
	println!("vetor[idx] = {}", vetor[idx]);

	//let idx2:usize = 6;	
	//vetor[idx2] = 4; // error: thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 6'

	idx = 5;
	match vetor.get(idx) {
		Some(x) => println!("vetor[{}] = {}", idx, x),
		None => println!("error, no such '{}' element", idx)
	}

	for x in &vetor { println!("{}", x); }


	let last_elem = vetor.pop(); // remove last element and return Option {Some(x) or None}
	println!("last elem is {:?}, vetor = {:?}", last_elem, vetor);

	let mut vetor2:Vec<u8> = Vec::new();
	let last_elem2 = vetor2.pop();
	println!("last elem 2 is {:?}, vetor2 = {:?}", last_elem2, vetor2);

	while let Some(x) = vetor.pop() {
		println!("{}", x)
	}

	while let Some(x) = vetor2.pop() {
		println!("{}", x)
	}


}


trait Animal {
	fn name(&self) -> &'static str; // Não implementado, portanto é obrigatória uma implementação de método no "impl"

	fn talk(&self) { // Implementado, portanto não é obrigatória uma implementação de método no "impl"
		println!("{} cannot talk", self.name());
	}
}

struct Human {
	name: &'static str
}

impl Animal for Human {

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
	fn name(&self) -> &'static str {
		self.name // name da struct Giraffe
	}
}


enum Creature {
	Human(Human),
	Giraffe(Giraffe)
}

pub fn vectors_of_different_objects () {
	let mut creatures = Vec::new();
	//creatures.push(Human{ name: "John" });
	//creatures.push(Giraffe{ name: "Gigi" }); // error: mismatched types, expected struct `Human`
	creatures.push(Creature::Human(
		Human{ name: "John" }
	));
	creatures.push(Creature::Giraffe(
		Giraffe{ name: "Gigi" }
	));

	for c in creatures {
		match c {
			Creature::Human(h) => h.talk(),
			Creature::Giraffe(g) => g.talk() 
		}
	}

	let mut animals:Vec<Box<Animal>> = Vec::new();
	animals.push(Box::new(
		Human{ name: "John" }
	));
	animals.push(Box::new(
		Giraffe{ name: "Gigi" }
	));

	for a in animals.iter() {
		a.talk();
	}


}