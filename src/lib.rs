// Copyright 2017 Gerald Haesendonck
// Licensed under the Academic Free License version 3.0

//! ntstream, a library to parse N-Triples on a per string basis.
//!
//! # Examples
//!
//! Here's a typical example. It sends an input string (can be a line from a
//! file for example) to the `triple_line()` method of the `parser`,
//! which returns an `Option<Triple>` if the input is valid, or a `None` or `ParseError`
//! if the input is a comment or an invalid triple respectively.
//!
//! If you are sure that the input should be a triple and no comment, white space,
//! or other rubbish, you can just use the `triple` method, which omits the
//! `Option` and directly returns a `ParseResult<Triple>`.
//!
//! ```
//!use ntstream::parser::triple_line;
//!use ntstream::{Subject, Predicate, Object};
//!
//!fn main() {
//!
//!  // Here's some input in n-triples format. Unicode escape sequences are resolved
//!  // so \u30AA becomes オ.
//!  let input = "_:subject <http://example.org/predicate> \"\\u30AAオブジェクト\".";
//!  
//!  // parse the input:
//!  let parse_result = triple_line(&input);
//!
//!  // The result contains an option, or an error when parsing the input failed.
//!  match parse_result {
//!
//!    // Ok if the input is a triple, a comment, an empty string or whitespace(s).
//!    Ok(triple_option) => {
//!      match triple_option {
//!        Some(triple) => { // a valid triple is found.
//!          match triple.subject {
//!            // In this case we expect a blank node label
//!            Subject::BNode(subject) => println!("Subject: blank node: {}", subject),
//!            _ => println!("Weird, a blank node is expected here.")
//!          };
//!          match triple.predicate {
//!            Predicate::IriRef(iri) => println!("Predicate: {}", iri)
//!          };
//!          match triple.object {
//!            Object::Lit(literal) => println!("Object: literal: {} ", literal.data),
//!            _ => println!("Weird, a literal is expected here.")
//!          }
//!        },
//!        None => { println!("Skipped [{}]", input); }
//!      };
//!    },
//!    // a parse error: the input is no valid triple, comment, empty string or whitespace(s)
//!    Err(error) => println!("{}\n{}", input, error),
//!  };
//!}
//! ```

/// Represents an RDF statement in the form of a triple.
#[derive(Debug)]
pub struct Triple {
	/// The subject part. This can be an IRI or a blank node.
	pub subject: Subject,
	/// The predicate part. This is an IRI.
	pub predicate: Predicate,
	/// The object part. This can be an IRI, a blank node or a literal.
	pub object: Object
}

/// Represents the subject part of a triple.
#[derive(Debug)]
pub enum Subject {
	/// The unescaped lexical form of the IRI.
	IriRef(String),
	BNode(String)
}

/// Represents the predicate part of a triple.
#[derive(Debug)]
pub enum Predicate {
	/// The unescaped lexical form of the IRI.
	IriRef(String)
}

/// Represents the object part of a triple.
#[derive(Debug)]
pub enum Object {
	/// The unescaped lexical form of the IRI.
	IriRef(String),
	BNode(String),
	/// The unescaped lexial form of the literal.
	Lit(Literal)
}

/// Represents either a type or a language.
#[derive(Debug)]
pub enum TypeLang {
	/// a language tag (e.g. `nl-be`).
	Lang(String),
	/// a type reference (e.g. `https://www.w3.org/2001/XMLSchema#float`).
	Type(String)
}

/// Represents an RDF literal.
#[derive(Debug)]
pub struct Literal {
	/// The literal part (e.g. `This is a literal`).
	pub data: String,
	/// The type or language of the literal.
	pub data_type: TypeLang
}

pub mod parser {
	//! Generated code.
	include!(concat!(env!("OUT_DIR"), "/ntstream.rs"));
}


mod tests;
