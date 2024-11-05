// XXX: clone allows duplicating parameters in operators
//
// Alternatively, this could be a huge enum.  If `b3` is not made extensible,
// there's no particular reason to have a public trait.  It'll remove the need
// for dynamic object fiddling at the cost of increasing parameter size.
//
// BEAST only has a handful of parameters: boolean, real, integer, and their
// list versions.  This is conducive to an enum
pub trait Parameter : Clone {}
