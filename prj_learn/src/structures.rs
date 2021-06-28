struct Point {
	x: f64,
	y: f64
}

struct Line {
	start: Point,
	end: Point
}

#[derive(Debug)] // to print
struct User {
	name: String,
	email: String,
	active: bool,
	admin: bool
}

fn build_user(name: String, email: String) -> User {
    User {
        email: email,
        name, // parameter name and the struct field name are exactly the same, we can use the field init shorthand syntax 
        active: true,
        admin: false
    }
}

// tuple structs
#[derive(Debug)]
struct Color(u8, u8, u8);

pub fn structures() {
	let p:Point = Point{x: 3.0, y: 4.0};
	println!("point p is at ({}, {})", p.x, p.y );

	let p2 = Point{x: 5.0, y: 10.0};

	let my_line = Line{start: p, end: p2};

	let user1 = build_user(String::from("Leandro"), String::from("leandro@example.com"));
	println!("{:#?}", user1);

	let user2 = User{
		name: String::from("Jose"),
		email: String::from("jose@example.com"),
		..user1
	};
	println!("{:#?}", user2);

	let black = Color(0, 0, 0);

	println!("{:?}", black);

}
