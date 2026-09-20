use super::cast::{ Identifier, Label };
use super::Token;

use super::identifier;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Label {
    let id: Identifier = identifier::parse(tokens);

    Label {
        id
    }
}