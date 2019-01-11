use super::*;

#[test]
fn test_parse() {
    let tokenizer= tokenizer::Tokenizer::new("( symbol \"string\" ( nestedfunc 44322 ) )".to_string());
    let expected = [
                                Token::OpenParen,
                                Token::Symbol("symbol".to_string()),
                                Token::Str("string".to_string()),
                                Token::OpenParen,
                                Token::Symbol("nestedfunc".to_string()),
                                Token::Int32(44322),
                                Token::CloseParen,
                                Token::CloseParen];

    for (real, exp) in tokenizer.tokens().zip(expected.iter()) {
        assert_eq!(real, *exp)
    }
}
