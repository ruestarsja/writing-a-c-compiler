use super::super::cast;
use super::super::tast;


pub(crate) fn generate(
    cast_unary_operator: cast::UnaryOperator
) -> tast::UnaryOperator {
    match cast_unary_operator {
        cast::UnaryOperator::Complement => tast::UnaryOperator::Complement,
        cast::UnaryOperator::Negate => tast::UnaryOperator::Negate,
        cast::UnaryOperator::Not => tast::UnaryOperator::Not,
        cast::UnaryOperator::PreIncrement |
        cast::UnaryOperator::PreDecrement |
        cast::UnaryOperator::PostIncrement |
        cast::UnaryOperator::PostDecrement => {
            panic!("internal error: no tast unary equivalent to increment and decrement operators (generate the appropriate binary and assignment operations manually instead)");
        }
    }
}