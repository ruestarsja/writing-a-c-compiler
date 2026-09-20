use super::tast;
use super::aast;

use super::identifier;


pub(super) fn generate(
    tast_value: tast::Value
) -> aast::Operand {
    match tast_value {
        
        tast::Value::Constant {
            int: tast_int
        } => {
            aast::Operand::Imm { int: tast_int }
        },

        tast::Value::Variable {
            name: tast_name
        } => {
            aast::Operand::Pseudo {
                name: identifier::generate(tast_name)
            }
        }

    }
}