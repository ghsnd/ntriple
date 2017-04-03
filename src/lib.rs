mod ntstream {
	enum TypeLang<String> {
		Lang(String),
		Type(String)
	}

	pub struct Literal {
		data: String,
		data_type: TypeLang<String>
	}

	pub mod parser {
		include!(concat!(env!("OUT_DIR"), "/ntstream.rs"));
	}
}

/*pub enum TypeLang<String> {
	Lang(String),
	Type(String)
}*/

#[cfg(test)]
mod tests {
	use ntstream::Literal;
	use ntstream::parser::*;

	#[test]
	fn test_uchar() {
		let result = UCHAR("\\u12ab").unwrap();
		println!("result: {:?}", result);
		assert_eq!(result, 'ካ');
	}

	#[test]
	fn test_blank_node_label() {
		let result = BLANK_NODE_LABEL("_:abc.a").unwrap();
		println!("result: {}", result);
	}

	#[test]
	fn test_string_literal_quote() {
		let result = STRING_LITERAL_QUOTE("\"Hola, dit is een \\\"string\\\" met u\\u12abicode! en een \\\\ \"").unwrap();
		println!("result: {}", result);
	}

	#[test]
	fn test_iriref() {
		let result = IRIREF("<http://dit/is/een/iriref#fragment>").unwrap();
		println!("result: {}", result);
	}

	#[test]
	fn test_langtag() {
		let result = LANGTAG("@nl-be").unwrap();
		println!("result: {}", result);
	}

	#[test]
	fn test_literal() {
		let literal1: Literal = literal("\"This has a language tag\"@en-gb").unwrap();
		
		//println!("result: {}", result);
		/*let result2 = literal("\"This has a type tag\"^^<http://example.org/some_type>").unwrap();
		println!("result2: {}", result2);*/
	}
}
