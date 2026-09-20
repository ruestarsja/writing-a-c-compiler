use super::tast;
use super::aast;


pub(super) fn generate(
    tast_identifier: tast::Identifier
) -> aast::Identifier {
    
    let id: String = tast_identifier.id;

    aast::Identifier {
        id
    }
}