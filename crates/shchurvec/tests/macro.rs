use shchurvec::{shchurvec, ShchurVec};

#[test]
fn empty() {
	// Copy
	let v: ShchurVec<usize> = shchurvec![];
	assert!(v.is_empty());

	// Clone
	let v: ShchurVec<Box<usize>> = shchurvec![];
	assert!(v.is_empty());
}

#[test]
fn list() {
	let v = shchurvec![1, 2, 3];
	assert_eq!([1, 2, 3], v);

	let v = shchurvec![Box::new(1), Box::new(2), Box::new(3)];
	assert_eq!([Box::new(1), Box::new(2), Box::new(3)], v);
}

#[test]
fn repeat() {
	let v = shchurvec![1; 10];
	assert_eq!([1; 10], v);

	let v = shchurvec![vec![20]; 20];
	assert_eq!(vec![vec![20]; 20], v);
}
