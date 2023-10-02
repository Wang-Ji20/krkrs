//! # Interpreter
//!
//! This module interprets `.ks` files. `Controller` calls this module to change
//! the state of the game.

/// parser module parses the `.ks` file and returns an iterator.
mod parser;

pub mod interpreter;
