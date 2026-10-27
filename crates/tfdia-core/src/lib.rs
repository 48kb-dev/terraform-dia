//! Core of terraform-dia: parsing Terraform sources into a model.
//!
//! This crate does no filesystem IO — callers pass file contents in.

pub mod model;
pub mod parser;

pub use model::{Edge, Model, Node, NodeKind};
pub use parser::{ParseError, SourceFile, parse};
