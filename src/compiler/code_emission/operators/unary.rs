use super::super::aast::UnaryOperator;


pub(crate) fn emit(
    aast_unary_operator: UnaryOperator
) -> String {
    match aast_unary_operator {

        UnaryOperator::Neg => {
            return "negl".to_owned();
        },

        UnaryOperator::Not => {
            return "notl".to_owned();
        }

    }
}