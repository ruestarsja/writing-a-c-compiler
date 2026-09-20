#[derive(Debug)]
pub(crate) struct Program {
    pub(crate) function_definition: Function
}

#[derive(Debug)]
pub(crate) struct Function {
    pub(crate) name: Identifier,
    pub(crate) body: Vec<BlockItem>
}

#[derive(Debug, Clone)]
pub(crate) struct Identifier {
    pub(crate) id: String
}

#[derive(Debug)]
pub(crate) enum BlockItem {
    Statement {
        statement: Statement
    },
    Declaration {
        declaration: Declaration
    }
}

#[derive(Debug)]
pub(crate) struct Label {
    pub(crate) id: Identifier
}

#[derive(Debug)]
pub(crate) struct Declaration {
    pub(crate) name: Identifier,
    pub(crate) init: Option<Expression>
}

#[derive(Debug)]
pub(crate) enum Statement {
    Return {
        labels: Vec<Label>,
        exp: Expression
    },
    Expression {
        labels: Vec<Label>,
        exp: Expression
    },
    If {
        labels: Vec<Label>,
        condition: Expression,
        then_statement: Box<Statement>,
        else_statement: Option<Box<Statement>>,
    },
    Goto {
        labels: Vec<Label>,
        target: Label
    },
    Null {
        labels: Vec<Label>,
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    Constant {
        int: i32
    },
    Var {
        id: Identifier
    },
    Unary {
        operator: UnaryOperator,
        factor: Box<Expression>
    },
    Binary {
        operator: BinaryOperator,
        lfactor: Box<Expression>,
        rfactor: Box<Expression>
    },
    Assignment {
        lvalue: Box<Expression>,
        exp: Box<Expression>
    },
    Conditional {
        condition: Box<Expression>,
        exp_if_true: Box<Expression>,
        exp_if_false: Box<Expression>
    }
}

#[derive(Debug, Clone)]
pub(crate) enum UnaryOperator {
    Complement,
    Negate,
    Not,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement
}

#[derive(Debug, Clone)]
pub(crate) enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
