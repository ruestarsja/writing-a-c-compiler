use super::aast::Instruction;

use super::conditioncode;
use super::identifier;
use super::operand;
use super::operators;


pub(super) fn emit(
    aast_instruction: Instruction
) -> String {
    match aast_instruction {

        Instruction::Mov {
            src,
            dst
        } => {
            return format!("\tmovl\t{},\t{}\n", operand::emit(src), operand::emit(dst));
        },

        Instruction::Unary {
            operator,
            operand
        } => {
            return format!("\t{}\t{}\n", operators::unary::emit(operator), operand::emit(operand));
        },

        Instruction::Binary {
            operator,
            src,
            dst
        } => {
            return format!(
                "\t{}\t{},\t{}\n",
                operators::binary::emit(operator),
                operand::emit(src),
                operand::emit(dst)
            );
        },

        Instruction::Cmp {
            loperand,
            roperand
        } => {
            return format!(
                "\tcmpl\t{},\t{}\n",
                operand::emit(loperand),
                operand::emit(roperand)
            );
        },

        Instruction::Idiv {
            operand
        } => {
            return format!(
                "\tidivl\t{}\n",
                operand::emit(operand)
            );
        },

        Instruction::Cdq => {
            return "\tcdq\n".to_owned();
        },

        Instruction::Jmp {
            target
        } => {
            return format!(
                "\tjmp\t.L{}\n",
                identifier::emit(target)
            );
        },

        Instruction::JmpCC {
            condition,
            target
        } => {
            return format!(
                "\tj{}\t.L{}\n",
                conditioncode::emit(condition),
                identifier::emit(target)
            );
        },

        Instruction::SetCC {
            condition,
            operand
        } => {
            return format!(
                "\tset{}\t{}\n",
                conditioncode::emit(condition),
                operand::emit_for_setcc(operand)
            );
        },

        Instruction::Label {
            id
        } => {
            return format!(
                ".L{}:\n",
                identifier::emit(id)
            );
        },

        Instruction::AllocateStack {
            bytes
        } => {
            return format!("\tsubq\t${},\t%rsp\n", bytes);
        },
        
        Instruction::Ret => {
            let mut return_code: String = String::new();

            return_code += "\tmovq\t%rbp,\t%rsp\n";
            return_code += "\tpopq\t%rbp\n";
            return_code += "\tret\n";

            return return_code;
        }

    }
}