//! Represents a single character in Jinko

use crate::instruction::{Instruction, InstrKind};
use super::Value;

pub struct JinkChar(char);

impl From<char> for JinkChar {
    fn from(c: char) -> Self {
        JinkChar(c)
    }
}

impl Value for JinkChar {}

impl Instruction for JinkChar {
    fn kind(&self) -> InstrKind {
        InstrKind::Expression
    }

    fn print(&self) -> String {
        self.0.to_string()
    }
}
