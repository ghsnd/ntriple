/**
 * Copyright 2017 Gerald Haesendonck
 * Licensed under the Academic Free License version 3.0
 **/

pub mod ntstream {

	#[derive(Debug)]
	pub struct Triple {
		pub subject: Subject,
		pub predicate: Predicate,
		pub object: Object
	}


	#[derive(Debug)]
	pub enum Subject {
		IriRef(String),
		BNode(String)
	}

	#[derive(Debug)]
	pub enum Predicate {
		IriRef(String)
	}

	#[derive(Debug)]
	pub enum Object {
		IriRef(String),
		BNode(String),
		Lit(Literal)
	}

	#[derive(Debug)]
	pub enum TypeLang {
		Lang(String),
		Type(String)
	}

	#[derive(Debug)]
	pub struct Literal {
		pub data: String,
		pub data_type: TypeLang
	}

	pub mod parser {
		include!(concat!(env!("OUT_DIR"), "/ntstream.rs"));
	}
}

mod tests;
