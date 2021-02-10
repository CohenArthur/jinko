//! The TypeCheck trait provides a mean for instructions to resolve their types, as well
//! as verify that the correct arguments are passed in the correct position to functions
//! or during type instantiations.
//! Let's take an example:
//!
//! ```
//! a = 15;
//! ```
//!
//! Here, the parser *knows* that 15 is an integer. So the type of `a` is fairly straight
//! forward and known early on. Now let's make a copy of `a`
//!
//! ```
//! b = a;
//! ```
//!
//! What is the type of `b`? The only way to know is to look at the type of `a`, and take
//! the same. Now, consider the following:
//!
//! ```
//! func get_string() -> string { "jinko" };
//! func concat(lhs: string, rhs: string) -> string { ... }
//! concat(get_string(), " loves you") // valid
//! concat(1, " loves you") // invalid, but how?
//! ```
//!
//! We need to ensure that all arguments passed to the `concat` function have the `string`
//! type. This also needs to apply to method calls, type instantiations and other complex
//! constructs.
//!
//! ```
//! "jinko".concat(" barks") // valid
//! 1.concat(3) // invalid
//!
//! 1 + 4 // valid
//! "string" + 5 // invalid
//! 1.0 + 6 // invalid
//! ```
//!
//! Much like the `Instruction` trait, this trait is meant to be used in an
//! AST-like way. Before execution, typecheck your interpreter to ensure its validity
//! and propagate type errors early and quickly.

use crate::JkError;
use crate::instruction::TypeDec;

pub trait TypeCheck {
    /// Return the type of the current instruction. If a clash or invalidity arises,
    /// raise a JkError::TypeCheck
    fn typecheck(&mut self) -> Result<TypeDec, JkError>;
}
