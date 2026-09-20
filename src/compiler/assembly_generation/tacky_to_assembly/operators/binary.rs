use super::super::tast;
use super::super::aast;


pub(crate) fn generate(
    tast_binary_operator: tast::BinaryOperator
) -> aast::BinaryOperator {
    match tast_binary_operator {

        tast::BinaryOperator::Add
            => aast::BinaryOperator::Add,
        tast::BinaryOperator::Subtract
            => aast::BinaryOperator::Sub,
        tast::BinaryOperator::Multiply
            => aast::BinaryOperator::Mult,
        tast::BinaryOperator::BitwiseAnd
            => aast::BinaryOperator::BitwiseAnd,
        tast::BinaryOperator::BitwiseOr
            => aast::BinaryOperator::BitwiseOr,
        tast::BinaryOperator::BitwiseXor
            => aast::BinaryOperator::BitwiseXor,
        tast::BinaryOperator::BitwiseLeftShift
            => aast::BinaryOperator::BitwiseLeftShift,
        tast::BinaryOperator::BitwiseRightShift
            => aast::BinaryOperator::BitwiseRightShift,
        
        tast::BinaryOperator::Divide |
        tast::BinaryOperator::Remainder => {
            panic!("internal error: no aast equivalent for division or remainder operators (use idiv instruction instead)");
        },

        tast::BinaryOperator::Equal |
        tast::BinaryOperator::NotEqual |
        tast::BinaryOperator::LessThan |
        tast::BinaryOperator::LessThanOrEqual |
        tast::BinaryOperator::GreaterThan |
        tast::BinaryOperator::GreaterThanOrEqual => {
            panic!("internal error: no aast equivalent for comparative operators (use conditional jumps and/or set instead)");
        }

    }
}


pub(crate) fn generate_condition_code(
    tast_binary_operator: tast::BinaryOperator
) -> aast::ConditionCode {
    match tast_binary_operator {

        tast::BinaryOperator::Equal
            => aast::ConditionCode::E,
        tast::BinaryOperator::NotEqual
            => aast::ConditionCode::NE,
        tast::BinaryOperator::LessThan
            => aast::ConditionCode::L,
        tast::BinaryOperator::LessThanOrEqual
            => aast::ConditionCode::LE,
        tast::BinaryOperator::GreaterThan
            => aast::ConditionCode::G,
        tast::BinaryOperator::GreaterThanOrEqual
            => aast::ConditionCode::GE,

        _ => {
            panic!("internal error: no condition code for most binary operators (use only for comparative operations)");
        }

    }
}