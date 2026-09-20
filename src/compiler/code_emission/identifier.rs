use super::aast::Identifier;


pub(super) fn emit(
    aast_identifier: Identifier
) -> String {
    return aast_identifier.id;
}