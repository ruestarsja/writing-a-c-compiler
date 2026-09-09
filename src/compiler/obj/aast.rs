#[derive(Debug)]
pub(crate) struct Program {
    pub(crate) function_definition: Function
}

#[derive(Debug)]
pub(crate) struct Function {
    pub(crate) name: Identifier,
    pub(crate) instructions: Vec<Instruction>
}

#[derive(Debug, Clone)]
pub(crate) struct Identifier {
    pub(crate) id: String
}

#[derive(Debug)]
pub(crate) enum Instruction {
    Mov {
        src: Operand,
        dst: Operand
    },
    Unary {
        operator: UnaryOperator,
        operand: Operand
    },
    Binary {
        operator: BinaryOperator,
        src: Operand,
        dst: Operand
    },
    Cmp {
        loperand: Operand,
        roperand: Operand
    },
    Idiv {
        operand: Operand
    },
    Cdq,
    Jmp {
        target: Identifier
    },
    JmpCC {
        condition: ConditionCode,
        target: Identifier
    },
    SetCC {
        condition: ConditionCode,
        operand: Operand
    },
    Label {
        id: Identifier
    },
    AllocateStack {
        bytes: u32
    },
    Ret
}

#[derive(Debug, Clone)]
pub(crate) enum Operand {
    Imm {
        int: i32
    },
    Reg {
        reg: Register
    },
    Pseudo {
        name: Identifier
    },
    Stack {
        offset: i32
    }
}

#[derive(Debug)]
pub(crate) enum UnaryOperator {
    Neg,
    Not
}

#[derive(Debug, Clone)]
pub(crate) enum BinaryOperator {
    Add,
    Sub,
    Mult,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift
}

#[derive(Debug)]
pub(crate) enum ConditionCode {
    E,
    NE,
    G,
    GE,
    L,
    LE
}

#[derive(Debug, Clone)]
pub(crate) enum Register {
    AX,
    CX,
    CL,
    DX,
    R10,
    R11
}
