use super::super::tast;
use super::super::aast;


pub(crate) fn generate(
    tast_unary_operator: tast::UnaryOperator
) -> aast::UnaryOperator {
    match tast_unary_operator {

        tast::UnaryOperator::Complement
            => aast::UnaryOperator::Not,
        tast::UnaryOperator::Negate
            => aast::UnaryOperator::Neg,

        tast::UnaryOperator::Not => {
            panic!("internal error: no aast equivalent for not operator (use cmp, mov, setcc instead)");
        }

    }
}