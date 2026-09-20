mod program;
mod function;
mod identifier;
mod instructions;
mod operand;
mod operators;

use super::tast;
use super::aast;


pub(super) fn main(
    tast: tast::Program
) -> aast::Program {
    program::generate(tast)
}