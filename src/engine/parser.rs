//! 正規表現の式をパースし、抽象構文木に変換
use std::{
    error::Error,
    fmt::{self, Display},
    mem::take
};

//! 抽象構文木を表現するための型
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>)
}

/// パースエラーを表すための型
#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char),
    InvalidRightParen(usize),
    NoPrev(usize),
    NoRightParen,
    Empty,
}