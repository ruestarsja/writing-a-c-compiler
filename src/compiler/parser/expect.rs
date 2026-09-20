use super::Token;

// FUNCTIONS

pub(super) fn expect(
    expected: Token,
    tokens: &mut Vec<Token>
) {
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected {:?} but got EOF", expected);
    }
    if expected != tokens[0] {
        panic!("syntax error while parsing: expected {:?} but got {:?}", expected, tokens[0]);
    }
    tokens.remove(0);
}