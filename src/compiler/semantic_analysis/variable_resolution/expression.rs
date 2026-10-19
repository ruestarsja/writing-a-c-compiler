use super::cast::{
    Expression,
    Identifier,
    UnaryOperator
};
use super::VariableMap;


pub(super) fn traverse(
    expression: Expression,
    variable_map: &mut VariableMap
) -> Expression {
    match expression {
        Expression::Constant {
            int
        } => {
            Expression::Constant {
                int
            }
        },
        Expression::Var {
            id
        } => {
            Expression::Var {
                id: Identifier {
                    id: match variable_map.get(&id.id) {
                        Some(uvi) => uvi.get_uid().to_owned(),
                        None => panic!("use of undeclared variable `{}`", id.id)
                    }
                }
            }
        },
        Expression::Unary {
            operator,
            factor
        } => {
            match operator {
                UnaryOperator::PreIncrement |
                UnaryOperator::PreDecrement |
                UnaryOperator::PostIncrement |
                UnaryOperator::PostDecrement => {
                    match *factor {
                        Expression::Var { id: _ } => {},
                        _ => panic!("cannot increment or decrement a(n) {:?} expression", *factor)
                    }
                },
                _ => {}
            }
            Expression::Unary {
                operator,
                factor: Box::new(
                    traverse(
                        *factor,
                        variable_map
                    )
                )
            }
        },
        Expression::Binary {
            operator,
            lfactor,
            rfactor
        } => {
            Expression::Binary {
                operator,
                lfactor: Box::new(
                    traverse(
                        *lfactor,
                        variable_map
                    )
                ),
                rfactor: Box::new(
                    traverse(
                        *rfactor,
                        variable_map
                    )
                )
            }
        },
        Expression::Assignment {
            lvalue,
            exp
        } => {
            match &*lvalue {
                Expression::Var { id: _ } => {},
                _ => panic!("invalid expression at left of assignment operator (expected variable name)")
            }
            Expression::Assignment {
                lvalue: Box::new(
                    traverse(
                        *lvalue,
                        variable_map
                    )
                ),
                exp: Box::new(
                    traverse(
                        *exp,
                        variable_map
                    )
                )
            }
        },
        Expression::Conditional {
            condition,
            exp_if_true,
            exp_if_false
        } => {
            Expression::Conditional {
                condition: Box::new(
                    traverse(
                        *condition,
                        variable_map
                    )
                ),
                exp_if_true: Box::new(
                    traverse(
                        *exp_if_true,
                        variable_map
                    )
                ),
                exp_if_false: Box::new(
                    traverse(
                        *exp_if_false,
                        variable_map
                    )
                )
            }
        }
    }
}