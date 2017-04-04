mod ntstream {

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

mod tests;
