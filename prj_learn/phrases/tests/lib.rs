#[cfg(test)]
mod tests {
	extern crate phrases;

	#[test]
	fn english_greenting_correct() {
		assert_eq!("hello", phrases::greetings::english::hello());
	}

	#[test]
	#[should_panic]
	fn english_greenting_incorrect() {
		assert_eq!("goodbye", phrases::greetings::english::hello());
	}

	#[test]
	#[ignore]
	fn english_greenting_ignore() {
		assert_eq!("goodbye", phrases::greetings::english::hello());
	}
}
