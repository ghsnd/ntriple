
// Copyright 2017 Gerald Haesendonck
// Licensed under the Academic Free License version 3.0

#[cfg(test)]
mod tests {
	use {Literal, Predicate, Subject, Object, Triple};
	use TypeLang::{Lang, Type};
	use parser::*;

	#[test]
	fn test_uchar() {
		let result = UCHAR("\\u12ab").unwrap();
		assert_eq!(result, 'ካ');
	}

	#[test]
	fn test_blank_node_label() {
		let result = BLANK_NODE_LABEL("_:abc.a").unwrap();
		assert_eq!(result, "abc.a");
	}

	#[test]
	fn test_string_literal_quote() {
		let result = STRING_LITERAL_QUOTE("\"Hola, dit is een \\\"string\\\" met u\\u12abicode! en een \\\\ \"").unwrap();
		assert_eq!(result, "Hola, dit is een \"string\" met uካicode! en een \\ ");
	}

	#[test]
	fn test_iriref() {
		let result = IRIREF("<http://dit/is/een/iriref#fragment>").unwrap();
		assert_eq!(result, "http://dit/is/een/iriref#fragment");
	}

	#[test]
	fn test_langtag() {
		let result = LANGTAG("@nl-be").unwrap();
		assert_eq!(result, "nl-be");
	}

	#[test]
	fn test_literal_lang() {
		let literal: Literal = literal("\"This has a language tag\"@en-gb").unwrap();
		assert_eq!(literal.data, "This has a language tag");
		let matches = match literal.data_type {
			Lang(lang) => lang == "en-gb",
			_ => false
		};
		assert_eq!(true, matches);
	}

	#[test]
	fn test_literal_type() {
		let literal = literal("\"This has a type tag\"^^<http://example.org/some_type>").unwrap();
		assert_eq!(literal.data, "This has a type tag");
		let matches = match literal.data_type {
			Type(tipe) => tipe == "http://example.org/some_type",
			_ => false
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_predicate() {
		let predicate: Predicate = predicate("<http://example.org/predicate>").unwrap();
		let matches = match predicate {
			Predicate::IriRef(iri) => iri == "http://example.org/predicate"
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_subject_iri() {
		let subject: Subject = subject("<http://example.org/subject>").unwrap();
		let matches = match subject {
			Subject::IriRef(iri) => iri == "http://example.org/subject",
			_ => false
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_subject_bnode() {
		let subject: Subject = subject("_:bnode").unwrap();
		let matches = match subject {
			Subject::BNode(bnode_label) => bnode_label == "bnode",
			_ => false
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_object_iri() {
		let object: Object = object("<http://example.org/object>").unwrap();
		let matches = match object {
			Object::IriRef(iri) => iri == "http://example.org/object",
			_ => false
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_object_bnode() {
		let object: Object = object("_:bnode").unwrap();
		let matches = match object {
			Object::BNode(bnode_label) => bnode_label == "bnode",
			_ => false
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_object_literal() {
		let object: Object = object("\"0.95\"^^<my::float::type>").unwrap();
		let matches = match object {
			Object::Lit(literal) => {
				let matches_type = match literal.data_type {
					Type(tipe) => tipe == "my::float::type",
					_ => false
				};
				matches_type && literal.data == "0.95"
			},
			_ => false
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_triple() {
		let triple: Triple = triple("<http://dbpedia.org/resource/Algeria> <http://www.w3.org/2000/01/rdf-schema#comment> \"Algeria (Arabic: الجزائر<\\u200E> al-Jazā'ir; Berber: ⵍⵣⵣⴰⵢⴻⵔ Dzayer), officially People's Democratic Republic of Algeria\"@en . # some comments").unwrap();
		println!("{:?}", triple);
		//let subject = triple.subject;
	}

	#[test]
	fn test_triple_line_triple() {
		let triple_option = triple_line("_:subject <http://example.org/predicate> \"object\".").unwrap();
		let matches = match triple_option {
			Some(_) => true,
			None => false
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_triple_line_comment() {
		let triple_option = triple_line(" # this is a nice comment").unwrap();
		let matches = match triple_option {
			Some(_) => false,
			None => true
		};
		assert_eq!(matches, true);
	}

	#[test]
	fn test_triple_line_empty() {
		let triple_option = triple_line("").unwrap();
		let matches = match triple_option {
			Some(_) => false,
			None => true
		};
		assert_eq!(matches, true);
	}

		#[test]
	fn test_triple_line_whitespaces() {
		let triple_option = triple_line("\t \t").unwrap();
		let matches = match triple_option {
			Some(_) => false,
			None => true
		};
		assert_eq!(matches, true);
	}
}
