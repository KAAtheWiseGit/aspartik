use linalg::RowMatrix;

const EPSILON: f64 = 1e-15;

macro_rules! assert_vector_eq {
	($left:expr, $right:expr) => {
		for i in 0..$right.len() {
			let diff = ($left[i] - $right[i]).abs();
			if diff > EPSILON {
				panic!(
					"Vectors are not equal:\n\t{:?}\n\t{:?}\n",
					$left, $right
				);
			}
		}
	};
}

#[test]
fn basic_eigenvalues() {
	let m = RowMatrix::from([[4.0, 1.0], [2.0, -1.0]]);
	assert_eq!([4.372281323269014, -1.3722813232690143], m.eigenvalues());

	let one_third = 1.0 / 3.0;
	let m = RowMatrix::from([
		[1.0, -one_third, -one_third, -one_third],
		[-one_third, 1.0, -one_third, -one_third],
		[-one_third, -one_third, 1.0, -one_third],
		[-one_third, -one_third, -one_third, 1.0],
	]);
	let ff = 4.0 / 3.0;
	assert_vector_eq!([0.0, ff, ff, ff], m.eigenvalues());
}
