#[derive(Debug)]
pub(crate) struct Program {
    pub(crate) function_definition: Function
}

#[derive(Debug)]
pub(crate) struct Function {
    pub(crate) name: Identifier,
    pub(crate) body: Vec<Instruction>
}

#[derive(Debug, Clone)]
pub(crate) struct Identifier {
    pub(crate) id: String
}

#[derive(Debug)]
pub(crate) enum Instruction {
    Return {
        val: Value
    },
    Unary {
        operator: UnaryOperator,
        src: Value,
        dst: Value
    },
    Binary {
        operator: BinaryOperator,
        lsrc: Value,
        rsrc: Value,
        dst: Value
    },
    Copy {
        src: Value,
        dst: Value
    },
    Jump {
        target: Identifier
    },
    JumpIfZero {
        condition: Value,
        target: Identifier
    },
    JumpIfNotZero {
        condition: Value,
        target: Identifier
    },
    Label {
        id: Identifier
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
    Constant {
        int: i32
    },
    Variable {
        name: Identifier
    }
}

#[derive(Debug)]
pub(crate) enum UnaryOperator {
    Complement,
    Negate,
    Not
}

#[derive(Debug)]
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
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual
}
