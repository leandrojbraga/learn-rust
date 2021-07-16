#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::ops::{Add, AddAssign, Neg};
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
struct Complex<T> {
	re: T,
	im: T
}

impl<T> Complex<T> {
	fn new(re:T, im: T) -> Complex<T> {
		Complex::<T> { re, im }
	}
}

impl<T> Add for Complex<T> 
  where T: Add<Output = T> {
	type Output = Complex<T>;

	// a+b
	//self+rhs
	fn add(self, rhs: Self) -> Self::Output {
		Complex {
			re: self.re + rhs.re,
			im: self.im + rhs.im
		}
	}
}

impl<T> AddAssign for Complex<T> 
  where T: AddAssign<T> {
	// a += b
	//self += rhs
	fn add_assign(&mut self, rhs: Self) {
		self.re += rhs.re;
		self.im += rhs.im;
	}
}

impl<T> Neg for Complex<T> 
  where T: Neg<Output = T> {
  	type Output = Complex<T>;

	// a += b
	//self += rhs
	fn neg(self) -> Self::Output {
		Complex {
			re: -self.re,
			im: -self.im
		}
	}
}

// derive in struct PartialEq ou implement:
// impl<T> PartialEq for Complex<T> 
//   where T: PartialEq {
// 	fn eq(&self, rhs: &Self) -> bool {
// 		self.re == rhs.re && self.im == rhs.im
// 	}

// 	fn ne(&self, rhs: &Self) -> bool {
// 		self.re != rhs.re || self.im != rhs.im
// 	}
// }

pub fn operator_overloading() {
	let mut a = Complex::new(1,2);
	let mut b = Complex::new(3,4);
	println!("{:?}", a + b); //error if not implement Add: cannot add `Complex<{T}>` to `Complex<{T}>`

	let mut c = Complex::new(1.0,2.0);
	let mut d = Complex::new(3.0,4.0);
	//println!("{:?}", c + d);

	c += d; // error if not implement AddAssign: binary assignment operation `+=` cannot be applied to type `Complex<{T}>`
	println!("{:?}", c);

	let mut e = Complex::new(5.0, 6.0);
	println!("{:?}", -e);

	let mut f = Complex::new(7.0, 8.0);
	let mut g = Complex::new(9.0, 8.0);
	println!("{:?}", f==g);
	println!("{:?}", f!=g);
	println!("{:?}", f==f);
	println!("{:?}", f!=f);
}