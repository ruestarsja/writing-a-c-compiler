use super::cast::Block;
use super::expect;
use super::Token;

use super::block_item;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Block {
    let mut block: Block = Block::new();
    expect(Token::OpenCurlyBracket, tokens);
    while tokens.first().is_some_and(
        |t| -> bool {*t != Token::CloseCurlyBracket}
    ) {
        block.push(
            block_item::parse(tokens)
        );
    }
    expect(Token::CloseCurlyBracket, tokens);
    block
}