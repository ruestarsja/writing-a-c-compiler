use super::aast::{ Operand, Register };


pub(super) fn emit(
    aast_operand: Operand
) -> String {
    match aast_operand {

        Operand::Imm {
            int
        } => {
            format!("${}", int)
        },

        Operand::Reg {
            reg
        } => {
            match reg {
                Register::AX
                    => "%eax".to_owned(),
                Register::CX
                    => "%ecx".to_owned(),
                Register::CL
                    => "%cl".to_owned(),
                Register::DX
                    => "%edx".to_owned(),
                Register::R10
                    => "%r10d".to_owned(),
                Register::R11
                    => "%r11d".to_owned()
            }
        },

        Operand::Stack {
            offset
        } => {
            format!("{}(%rbp)", offset)
        },

        Operand::Pseudo { name: _ } => panic!("internal error: pseudoregister not replaced upon code emission")

    }
}


pub(super) fn emit_for_setcc(
    aast_operand: Operand
) -> String {
    match aast_operand {

        Operand::Reg {
            reg
        } => {
            match reg {
                Register::AX
                    => "%al".to_owned(),
                Register::CX |
                Register::CL
                    => "%cl".to_owned(),
                Register::DX
                    => "%dl".to_owned(),
                Register::R10
                    => "%r10b".to_owned(),
                Register::R11
                    => "%r11b".to_owned()
            }
        },

        _ => emit(aast_operand)
         
    }
}