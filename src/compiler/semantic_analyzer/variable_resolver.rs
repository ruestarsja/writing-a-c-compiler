use std::collections::HashMap;

use super::super::obj::cast;

static mut LOCAL_VARIABLE_COUNTER: u32 = 0;

// FUNCTIONS

pub(super) fn main(
    ast: cast::Program
) -> cast::Program {
    let mut variable_map: HashMap<String, String> = HashMap::new();
    let program: cast::Program = traverse_program(ast, &mut variable_map);
    return program;
}

fn traverse_program(
    program: cast::Program,
    variable_map: &mut HashMap<String, String>
) -> cast::Program {
    cast::Program {
        function_definition: traverse_function(program.function_definition, variable_map)
    }
}

fn traverse_function(
    function: cast::Function,
    variable_map: &mut HashMap<String, String>
) -> cast::Function {
    let mut function_body: Vec<cast::BlockItem> = vec![];
    for block_item in function.body {
        function_body.push(traverse_block_item(block_item, variable_map));
    }
    cast::Function {
        name: function.name,
        body: function_body
    }
}

fn traverse_block_item(
    block_item: cast::BlockItem,
    variable_map: &mut HashMap<String, String>
) -> cast::BlockItem {
    match block_item {
        cast::BlockItem::Statement { statement } => {
            cast::BlockItem::Statement {
                statement: traverse_statement(statement, variable_map)
            }
        },
        cast::BlockItem::Declaration { declaration } => {
            cast::BlockItem::Declaration {
                declaration: traverse_declaration(declaration, variable_map)
            }
        }
    }
}

fn traverse_statement(
    statement: cast::Statement,
    variable_map: &mut HashMap<String, String>
) -> cast::Statement {
    match statement {
        cast::Statement::Return { exp } => {
            cast::Statement::Return {
                exp: traverse_expression(exp, variable_map)
            }
        },
        cast::Statement::Expression { exp } => {
            cast::Statement::Expression {
                exp: traverse_expression(exp, variable_map)
            }
        },
        cast::Statement::If { condition, then_statement, else_statement} => {
            cast::Statement::If {
                condition: traverse_expression(condition, variable_map),
                then_statement: Box::new(traverse_statement(*then_statement, variable_map)),
                else_statement: match else_statement {
                    Some(statement) => Some(Box::new(
                        traverse_statement(*statement, variable_map)
                    )),
                    None => None
                }
            }
        },
        cast::Statement::Null => cast::Statement::Null
    }
}

fn traverse_declaration(
    declaration: cast::Declaration,
    variable_map: &mut HashMap<String, String>
) -> cast::Declaration {
    if variable_map.get(&declaration.name.id).is_some() {
        panic!("cannot declare the same variable twice");
    }
    let unique_id: String = make_unique_id(&declaration.name.id);
    variable_map.insert(declaration.name.id, unique_id.clone());
    cast::Declaration {
        name: cast::Identifier{ id: unique_id },
        init: match declaration.init {
            Some(exp) => {
                Some(traverse_expression(exp, variable_map))
            },
            None => None
        }
    }
}

fn traverse_expression(
    expression: cast::Expression,
    variable_map: &mut HashMap<String, String>
) -> cast::Expression {
    match expression {
        cast::Expression::Constant { int } => {
            cast::Expression::Constant { int }
        },
        cast::Expression::Var { id } => {
            cast::Expression::Var {
                id: cast::Identifier {
                    id: match variable_map.get(&id.id) {
                        Some(unique_id) => {
                            unique_id.to_owned()
                        },
                        None => {
                            panic!("use of uninitialized variable `{}`", id.id);
                        }
                    }
                }
            }
        },
        cast::Expression::Unary { operator, factor } => {
            match operator {
                cast::UnaryOperator::PreIncrement |
                cast::UnaryOperator::PreDecrement |
                cast::UnaryOperator::PostIncrement |
                cast::UnaryOperator::PostDecrement => {
                    match *factor {
                        cast::Expression::Assignment { lvalue: _, exp: _ } |
                        cast::Expression::Binary { operator: _, lfactor: _, rfactor: _ } |
                        cast::Expression::Constant { int: _ } |
                        cast::Expression::Unary { operator: _, factor: _ } |
                        cast::Expression::Conditional { condition: _, then_expression: _, else_expression: _ } => {
                            panic!("cannot increment or decrement a(n) {:?} expression", *factor)
                        },
                        cast::Expression::Var { id: _ } => {}
                    }
                },
                cast::UnaryOperator::Complement |
                cast::UnaryOperator::Negate |
                cast::UnaryOperator::Not => {}
            }
            cast::Expression::Unary {
                operator,
                factor: Box::new(traverse_expression(*factor, variable_map))
            }
        },
        cast::Expression::Binary { operator, lfactor, rfactor } => {
            cast::Expression::Binary {
                operator,
                lfactor: Box::new(traverse_expression(*lfactor, variable_map)),
                rfactor: Box::new(traverse_expression(*rfactor, variable_map))
            }
        },
        cast::Expression::Assignment { lvalue, exp } => {
            match &*lvalue {
                cast::Expression::Var { id: _ } => {
                    cast::Expression::Assignment {
                        lvalue: Box::new(traverse_expression(*lvalue, variable_map)),
                        exp: Box::new(traverse_expression(*exp, variable_map))
                    }
                },
                cast::Expression::Constant { int: _ } |
                cast::Expression::Unary { operator: _, factor: _ } |
                cast::Expression::Binary { operator: _, lfactor: _, rfactor: _ } |
                cast::Expression::Assignment { lvalue: _, exp: _ } |
                cast::Expression::Conditional { condition: _, then_expression: _, else_expression: _ } => {
                    panic!("invalid expression at left of assignment operator (expected variable name)");
                }
            }
        },
        cast::Expression::Conditional { condition, then_expression, else_expression } => {
            cast::Expression::Conditional {
                condition: Box::new(traverse_expression(*condition, variable_map)),
                then_expression: Box::new(traverse_expression(*then_expression, variable_map)),
                else_expression: Box::new(traverse_expression(*else_expression, variable_map))
            }
        }
    }
}

fn make_unique_id(
    user_defined_variable_id: &str
) -> String {
    let count: u32;
    unsafe {
        count = LOCAL_VARIABLE_COUNTER;
        LOCAL_VARIABLE_COUNTER += 1;
    }
    return format!(
        "$loc{}.{}",
        count,
        user_defined_variable_id
    );
}
