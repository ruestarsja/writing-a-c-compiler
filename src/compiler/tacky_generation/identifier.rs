use super::cast;
use super::tast;


pub(super) fn generate(
    cast_identifier: cast::Identifier
) -> tast::Identifier {
    tast::Identifier {
        id: cast_identifier.id
    }
}