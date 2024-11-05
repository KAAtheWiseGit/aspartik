// If the entire state is clone, it can be duplicated for each operator action.
// The issue is that `State` will likely be pretty big, and copying it each time
// is wasteful.
//
// A better strategy would be to copy individual parameters.  But this requires
// merging old state with the proposed parameter for likelihood calculations.
pub struct State {}

impl State {
	// TODO: how do we index parameters?
	// TODO: what do we return?  A dyn box is indirection and might be
	// inefficient
	// pub fn get_parameter(id) -> Parameter

	// Distinct
	// pub fn get_tree() -> Tree
}
