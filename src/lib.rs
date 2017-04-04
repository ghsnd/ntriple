mod ntstream {

	#[derive(Debug)]
	pub enum Subject {
		IriRef(String),
		BlankNode(String)
	}

	#[derive(Debug)]
	pub enum Predicate {
		IriRef(String)
	}

	#[derive(Debug)]
	pub enum TypeLang {
		Lang(String),
		Type(String)
	}

	#[derive(Debug)]
	pub struct Literal {
		data: String,
		data_type: TypeLang
	}

	impl Literal {
		pub fn get_data(&self) -> &str {
			&self.data
		}

		pub fn get_type(&self) -> &TypeLang {
			&self.data_type
		}

	}

	pub mod parser {
		include!(concat!(env!("OUT_DIR"), "/ntstream.rs"));
	}
}

#[cfg(test)]
mod tests {
	use ntstream::Literal;
	use ntstream::Predicate;
	use ntstream::Subject;
	use ntstream::TypeLang::{self, Lang, Type};
	use ntstream::parser::*;

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
		assert_eq!(literal.get_data(), "This has a language tag");
		let type_lang: &TypeLang = literal.get_type();
		println!("type: {:?}", type_lang);
		//if let Lang("en
		let matches = match type_lang {
			&Lang(ref lang) => lang == "en-gb",
			_ => false
		};
		assert_eq!(true, matches);
	}

	#[test]
	fn test_literal_type() {
		let literal = literal("\"This has a type tag\"^^<http://example.org/some_type>").unwrap();
		assert_eq!(literal.get_data(), "This has a type tag");
		let type_lang: &TypeLang = literal.get_type();
		let matches = match type_lang {
			&Type(ref tipe) => tipe == "http://example.org/some_type",
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
			Subject::BlankNode(bnode_label) => bnode_label == "bnode",
			_ => false
		};
		assert_eq!(matches, true);
	}
}
