#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(bare_trait_objects)]

// Student* --- *Course

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Student {
	name: String,
	courses: Vec<Rc<RefCell<Course>>>
}

impl Student {
	fn new(name: &str) -> Student {
		Student {
			name: name.into(),
			courses: Vec::new()
		}
	}
}

#[derive(Debug)]
struct Course {
	name: String,
	students: Vec<Rc<RefCell<Student>>>
}

impl Course {
	fn new(name: &str) -> Course {
		Course {
			name: name.into(),
			students: Vec::new()
		}
	}
	fn add_student(
		course: Rc<RefCell<Course>>,
		student: Rc<RefCell<Student>>,
	) {
		student.borrow_mut().courses.push(course.clone());
		course.borrow_mut().students.push(student);
	}
}


fn rc_and_refcell() {
	let john = Rc::new(
		RefCell::new(
			Student::new("John")
		)
	);
	let jane = Rc::new(
		RefCell::new(
			Student::new("Jane")
		)
	);


	let rust_course = Course::new("Rust");
	let course = Rc::new(
		RefCell::new(
			rust_course
		)
	);

	Course::add_student(course.clone(), john);
	Course::add_student(course.clone(), jane);
	//println!("{:?}", course)
}

#[derive(Debug)]
struct Student2 { name: String }

impl Student2 {
	fn courses(&self, plataform: &Plataform) -> Vec<String> {
		plataform.enrollments.iter()
			.filter(|&e| e.student.name == self.name)
			.map(|e| e.course.name.clone())
			.collect()
	}
}

#[derive(Debug)]
struct Course2 { name: String }

impl Course2 {
	fn students(&self, plataform: &Plataform) -> Vec<String> {
		plataform.enrollments.iter()
			.filter(|&e| e.course.name == self.name)
			.map(|e| e.student.name.clone())
			.collect()
	}
}

#[derive(Debug)]
struct Enrollment<'a> {
	student: &'a Student2,
	course: &'a Course2 
}

impl<'a> Enrollment<'a> {
	fn new(student: &'a Student2, course: &'a Course2) -> Enrollment<'a> {
		Enrollment { student, course }
	}
}

#[derive(Debug)]
struct Plataform<'a> {
	enrollments: Vec<Enrollment<'a>>
}

impl<'a> Plataform<'a>{
	fn new() -> Plataform<'a> {
		Plataform {
			enrollments: Vec::new()
		}
	}

	fn enroll(&mut self,
		student: &'a Student2,
		course: &'a Course2) {

		self.enrollments.push(
			Enrollment::new(student, course)
		)
	}
}

fn struct_support() {
	let john = Student2{ name: "John".into() };
	let jane = Student2{ name: "Jane".into() };

	let rust_course = Course2{ name: "Rust".into() };

	let mut p = Plataform::new();
	p.enroll(&john, &rust_course);
	p.enroll(&jane, &rust_course);

	println!("{:#?}", p);

	
	for c in john.courses(&p).iter() {
		println!("{} is taking {}", john.name, c);
	}

	println!("Student list of {} course:", rust_course.name);
	for s in rust_course.students(&p).iter() {
		println!("- {}", s);
	}
}


pub fn circular_references() {
	rc_and_refcell();
	struct_support();
}