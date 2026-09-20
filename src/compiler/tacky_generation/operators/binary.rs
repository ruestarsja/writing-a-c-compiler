use super::super::cast;
use super::super::tast;


pub(crate) fn generate(
    cast_binary_operator: cast::BinaryOperator
) -> tast::BinaryOperator {
    match cast_binary_operator {
        cast::BinaryOperator::Add => tast::BinaryOperator::Add,
        cast::BinaryOperator::Subtract => tast::BinaryOperator::Subtract,
        cast::BinaryOperator::Multiply => tast::BinaryOperator::Multiply,
        cast::BinaryOperator::Divide => tast::BinaryOperator::Divide,
        cast::BinaryOperator::Remainder => tast::BinaryOperator::Remainder,
        cast::BinaryOperator::BitwiseAnd => tast::BinaryOperator::BitwiseAnd,
        cast::BinaryOperator::BitwiseOr => tast::BinaryOperator::BitwiseOr,
        cast::BinaryOperator::BitwiseXor => tast::BinaryOperator::BitwiseXor,
        cast::BinaryOperator::BitwiseLeftShift => tast::BinaryOperator::BitwiseLeftShift,
        cast::BinaryOperator::BitwiseRightShift => tast::BinaryOperator::BitwiseRightShift,
        cast::BinaryOperator::And |
        cast::BinaryOperator::Or => panic!("internal error: no tast equivalent to cast and/or operations (use custom short-circuit logic instead)"),
        cast::BinaryOperator::Equal => tast::BinaryOperator::Equal,
        cast::BinaryOperator::NotEqual => tast::BinaryOperator::NotEqual,
        cast::BinaryOperator::LessThan => tast::BinaryOperator::LessThan,
        cast::BinaryOperator::LessThanOrEqual => tast::BinaryOperator::LessThanOrEqual,
        cast::BinaryOperator::GreaterThan => tast::BinaryOperator::GreaterThan,
        cast::BinaryOperator::GreaterThanOrEqual => tast::BinaryOperator::GreaterThanOrEqual
    }
}


pub(crate) fn generate_from_unary(
    cast_unary_operator: cast::UnaryOperator
) -> tast::BinaryOperator {
    match cast_unary_operator {
        cast::UnaryOperator::PreIncrement |
        cast::UnaryOperator::PostIncrement => tast::BinaryOperator::Add,
        cast::UnaryOperator::PreDecrement |
        cast::UnaryOperator::PostDecrement => tast::BinaryOperator::Subtract,
        _ => panic!("internal error: `binary::generate_from_unary()` called for unary operator with no tast binary equivalent (use unary::generate instead)")
    }
}