extern crate rocks_lang;

use rocks_lang::token::{Token, Type, Location};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn create_token() {
    let token = Token::new(
        Type::LeftParen,
        "(".to_string(),
        None,
        Location::new(1, 3),
    );

    assert_eq!(token.r#type, Type::LeftParen);
    assert_eq!(token.lexeme, "(");
    assert_eq!(token.literal, None);
    assert_eq!(token.location.line, 1);
    assert_eq!(token.location.column, 3);
}

#[test]
fn create_token_from_str() {
    let token = Token::from("init");

    assert_eq!(token.r#type, Type::Identifier);
    assert_eq!(token.lexeme, "init");
    assert_eq!(token.literal, None);
    assert_eq!(token.location.line, 0);
    assert_eq!(token.location.column, 0);
}

#[test]
fn display_token() {
    let token = Token::new(
        Type::LeftParen,
        "(".to_string(),
        None,
        Location::new(1, 3),
    );

    assert_eq!(format!("{}", token), "LeftParen ( None @ [1:3]");
}

#[test]
fn same_hash_for_same_token() {
    let token = Token::from("init");
    let token_copy = Token::from("init");

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    let hash = hasher.finish();

    let mut hasher_copy = DefaultHasher::new();
    token_copy.hash(&mut hasher_copy);
    let hash_copy = hasher_copy.finish();

    assert_eq!(hash, hash_copy);
}

#[test]
fn different_hash_for_different_name() {
    let token = Token::from("init");
    let token_copy = Token::from("init2");

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    let hash = hasher.finish();

    let mut hasher_copy = DefaultHasher::new();
    token_copy.hash(&mut hasher_copy);
    let hash_copy = hasher_copy.finish();

    assert_ne!(hash, hash_copy);
}

#[test]
fn different_hash_for_different_type() {
    let token = Token::new(
        Type::Semicolon,
        ";".to_string(),
        None,
        Location::new(1, 3),
    );
    let token_copy = Token::new(
        Type::LeftParen,
        "(".to_string(),
        None,
        Location::new(1, 3),
    );

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    let hash = hasher.finish();

    let mut hasher_copy = DefaultHasher::new();
    token_copy.hash(&mut hasher_copy);
    let hash_copy = hasher_copy.finish();

    assert_ne!(hash, hash_copy);
}

#[test]
fn different_hash_for_different_row() {
    let token = Token::new(
        Type::Semicolon,
        ";".to_string(),
        None,
        Location::new(2, 4),
    );
    let token_copy = Token::new(
        Type::Semicolon,
        ";".to_string(),
        None,
        Location::new(1, 4),
    );

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    let hash = hasher.finish();

    let mut hasher_copy = DefaultHasher::new();
    token_copy.hash(&mut hasher_copy);
    let hash_copy = hasher_copy.finish();

    assert_ne!(hash, hash_copy);
}

#[test]
fn different_hash_for_different_column() {
    let token = Token::new(
        Type::Semicolon,
        ";".to_string(),
        None,
        Location::new(1, 3),
    );
    let token_copy = Token::new(
        Type::Semicolon,
        ";".to_string(),
        None,
        Location::new(1, 4),
    );

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    let hash = hasher.finish();

    let mut hasher_copy = DefaultHasher::new();
    token_copy.hash(&mut hasher_copy);
    let hash_copy = hasher_copy.finish();

    assert_ne!(hash, hash_copy);
}
