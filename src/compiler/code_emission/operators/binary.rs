use super::super::aast::BinaryOperator;


pub(crate) fn emit(
    aast_binary_operator: BinaryOperator
) -> String {
    match aast_binary_operator {

        BinaryOperator::Add => {
            return "addl".to_owned()
        },

        BinaryOperator::Sub => {
            return "subl".to_owned()
        },

        BinaryOperator::Mult => {
            return "imull".to_owned()
        },

        BinaryOperator::BitwiseAnd => {
            return "andl".to_owned()
        },

        BinaryOperator::BitwiseOr => {
            return "orl".to_owned()
        },

        BinaryOperator::BitwiseXor => {
            return "xorl".to_owned()
        },

        BinaryOperator::BitwiseLeftShift => {
            return "sall".to_owned();
        },

        BinaryOperator::BitwiseRightShift => {
            return "sarl".to_owned();
        }

    }
}