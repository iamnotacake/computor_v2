#![feature(box_syntax, box_patterns, slice_patterns)]

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}

pub mod expr;
pub use expr::Expr;
