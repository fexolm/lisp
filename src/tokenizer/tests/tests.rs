use crate::tokenizer::tokenizer;
use crate::tokenizer::tokenizer::Token;
use crate::tokenizer::tests::helpers::test_suffix_helper;
use crate::tokenizer::tests::helpers::check_correctness_helper;

#[test]
// simple test, spaces everywhere
fn test_spaces_1() {
    let program = r#"
                            ( symbol "string" ( nestedfunc 44322 ) )
                        "#.to_string();
    let tokenizer= tokenizer::Tokenizer::new(program);
    let expected = [
        Token::OpenParen,
        Token::Symbol("symbol".to_string()),
        Token::Str("string".to_string()),
        Token::OpenParen,
        Token::Symbol("nestedfunc".to_string()),
        Token::Int32(44322),
        Token::CloseParen,
        Token::CloseParen];

    check_correctness_helper(&expected, &tokenizer);
}

#[test]
// parenthesis without spaces
fn test_spaces_2() {
    let program = r#"
                            (symbol "string" (nestedfunc 44322))
                        "#.to_string();
    let tokenizer= tokenizer::Tokenizer::new(program);
    let expected = [
        Token::OpenParen,
        Token::Symbol("symbol".to_string()),
        Token::Str("string".to_string()),
        Token::OpenParen,
        Token::Symbol("nestedfunc".to_string()),
        Token::Int32(44322),
        Token::CloseParen,
        Token::CloseParen];
    check_correctness_helper(&expected, &tokenizer);
}

// parenthesis with indentation
#[test]
fn test_spaces_3() {
    let program = r#"
                            (
                                symbol
                                "string"
                                (
                                    nestedfunc
                                    44322
                                )
                            )
                        "#.to_string();
    let tokenizer= tokenizer::Tokenizer::new(program);
    let expected = [
        Token::OpenParen,
        Token::Symbol("symbol".to_string()),
        Token::Str("string".to_string()),
        Token::OpenParen,
        Token::Symbol("nestedfunc".to_string()),
        Token::Int32(44322),
        Token::CloseParen,
        Token::CloseParen];
    check_correctness_helper(&expected, &tokenizer);

}

#[test]
// test with floating number without suffix
fn test_float() {
    let program = r#"
                            ( setq symbol 3.14 )
                        "#.to_string();
    let tokenizer= tokenizer::Tokenizer::new(program);
    let expected = [
        Token::OpenParen,
        Token::Symbol("symbol".to_string()),
        Token::Float64(3.14f64),
        Token::CloseParen,
    ];
    check_correctness_helper(&expected, &tokenizer);
}

#[test]
//test for all suffixed numbers
fn test_suffixes() {
    test_suffix_helper(8i8);
    test_suffix_helper(16i16);
    test_suffix_helper(32i32);
    test_suffix_helper(64i64);
    test_suffix_helper(32.32f32);
    test_suffix_helper(64.64f64);
}