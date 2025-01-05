use std::{cell::RefCell, mem::drop};

use shchurvec::{shchurvec, ShchurVec};

thread_local! {
	static COUNT: RefCell<usize> = RefCell::new(0);
}

#[derive(Debug, PartialEq)]
struct Leak {}

impl Leak {
	fn new() -> Leak {
		COUNT.with_borrow_mut(|c| {
			println!("new {} -> {}", c, *c + 1);
			*c += 1;
		});

		Leak {}
	}
}

impl Clone for Leak {
	fn clone(&self) -> Self {
		COUNT.with_borrow_mut(|c| {
			println!("clone {} -> {}", c, *c + 1);
			*c += 1;
		});

		Leak {}
	}
}

impl Drop for Leak {
	fn drop(&mut self) {
		COUNT.with_borrow_mut(|c| {
			println!("drop {} -> {}", c, *c - 1);
			*c -= 1;
		});
	}
}

macro_rules! assert_no_leak {
	($v:ident) => {{
		drop($v);
		COUNT.with_borrow(|v| assert_eq!(0, *v, "Leak!"));
	}};
}

#[test]
fn basic() {
	let mut v = ShchurVec::new();
	v.push(Leak::new());
	assert_no_leak!(v);
}

#[test]
fn push() {
	let mut v = ShchurVec::new();
	v.push(Leak::new());
	v.push(Leak::new());
	v.push(Leak::new());
	assert_no_leak!(v);
}

#[test]
fn r#macro() {
	let v = shchurvec![Leak::new()];
	assert_no_leak!(v);

	let v = shchurvec![Leak::new(); 10];
	assert_no_leak!(v);
}

#[test]
fn set() {
	let mut v = shchurvec![Leak::new()];
	v.set(0, Leak::new());
	v.set(0, Leak::new());
	assert_no_leak!(v);
}
