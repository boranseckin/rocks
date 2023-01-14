#![allow(dead_code)]

use std::fmt::Debug;

use crate::token::Token;

pub trait Expr {
    fn serialize(&self) -> String;
}

impl Debug for dyn Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expr::{}", self.serialize())
    }
}

impl PartialEq for dyn Expr {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
pub enum Literal {
    True,
    False,
    Nil,
    Number(f32),
    String(String),
}

impl Expr for Literal {
    fn serialize(&self) -> String {
        format!("Literal::{:#?}", self)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum UnaryPrefix {
    Bang,
    Minus, 
}

#[derive(Debug)]
pub struct Unary {
    prefix: UnaryPrefix,
    operator: Token,
}

impl Expr for Unary {
    fn serialize(&self) -> String {
        format!("{:#?}", self)
    }
}

impl Unary {
    pub fn new(
        prefix: UnaryPrefix,
        operator: Token,
    ) -> Unary {
        Unary { prefix, operator }
    }
}

#[derive(Debug)]
pub struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn serialize(&self) -> String {
        format!("{:#?}", self)
    }
}

impl Binary {
    pub fn new(
        left: Box<dyn Expr>,
        operator: Token,
        right: Box<dyn Expr>
    ) -> Binary {
        Binary { left, operator, right }
    }
}

#[derive(Debug)]
pub struct Grouping {
    expression: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn serialize(&self) -> String {
        format!("{:#?}", self)
    }
}

impl Grouping {
    pub fn new(
        expression: Box<dyn Expr>
    ) -> Grouping {
        Grouping { expression }
    }
}

#[cfg(test)]
mod test {
    use crate::token::Type;

    use super::*;

    #[test]
    fn create_unary() {
        let token = Token::new(Type::TRUE, String::from("true"), None, 1);
        let unary = Unary::new(UnaryPrefix::Bang, token.clone());

        assert_eq!(unary.prefix, UnaryPrefix::Bang);
        assert_eq!(unary.operator, token);
    }

    // #[test]
    // fn create_binary() {
    //     let token = Token::new(Type::AND, String::from("and"), None, 1);
    //     let literal1 = Literal::True;
    //     let literal2 = Literal::False;
    //     let binary = Binary::new(Box::new(literal1), token.clone(), Box::new(literal2));
    //
    //     // dbg!(binary);
    //     // assert_eq!(binary.left, Box::new(Literal::True));
    //     assert_eq!(binary.operator, token);
    // }
    //
    // #[test]
    // fn create_expr() {
    //     let tok1 = Token::new(crate::token::Type::ELSE, String::from("else"), None, 1);
    //     let tok2 = Token::new(crate::token::Type::IF, String::from("if"), None, 1);
    //     let tok3 = Token::new(crate::token::Type::AND, String::from("and"), None, 1);
    //     let bin2 = Unary::new(UnaryPrefix::Bang, tok2);
    //     let bin1 = Unary::new(UnaryPrefix::Bang, tok1);
    //     let gro = Grouping::new(Box::new(bin2));
    //
    //     let una = Binary::new(Box::new(bin1), tok3, Box::new(gro));
    //     // println!("{:#?}", una);
    // }
}

