#![feature(box_patterns, std_misc)]

extern crate "parse-regex" as parse_regex;
extern crate automata;

pub mod lexer;

pub use lexer::Lexer;
