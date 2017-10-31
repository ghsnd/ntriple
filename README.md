# ntriple
[![Build Status](https://travis-ci.org/ghsnd/ntriple.svg?branch=master)](https://travis-ci.org/ghsnd/ntriple)

A parser that parses a single RDF statement, in RDF [N-Triples](https://www.w3.org/TR/n-triples/) format.

It is written in [Rust](https://www.rust-lang.org/en-US/) and uses [rust-peg](https://github.com/kevinmehall/rust-peg)
for parsing the triple to an object structure. Literals are decomposed into
their [RDF lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form) and language or data type.

The purpose of this library is to process triples in a streaming way, so it doesn't build
an [RDF Graph](https://www.w3.org/TR/2014/REC-rdf11-concepts-20140225/#section-rdf-graph).
If you are looking for libraries that do, take a look at
[rust-ntriples](https://github.com/enex/rust-ntriples/) or [rome](https://github.com/vandenoever/rome).

## Example:

```rust

extern crate ntriple;

use ntriple::parser::triple_line;
use ntriple::{Subject, Predicate, Object};

fn main() {
  
  // Here's some input in n-triples format. Unicode escape sequences are resolved
  // so \u30AA becomes オ.
  let input = "_:subject <http://example.org/predicate> \"\\u30AAオブジェクト\".";
  
  // parse the input:
  let parse_result = triple_line(&input);
  
  // The result contains an option, or an error when parsing the input failed.
  match parse_result {
  
    // Ok if the input is a triple, a comment, an empty string or whitespace(s).
    Ok(triple_option) => {
      match triple_option {
        Some(triple) => { // a valid triple is found.
          match triple.subject {
            // In this case we expect a blank node label
            Subject::BNode(subject) => println!("Subject: blank node: {}", subject),
            _ => println!("Weird, a blank node is expected here.")
          };
          match triple.predicate {
            Predicate::IriRef(iri) => println!("Predicate: {}", iri)
          };
          match triple.object {
            Object::Lit(literal) => println!("Object: literal: {} ", literal.data),
            _ => println!("Weird, a literal is expected here.")
          }
        },
        None => { println!("Skipped [{}]", input); }
      };
    },
    // a parse error: the input is no valid triple, comment, empty string or whitespace(s)
    Err(error) => println!("{}\n{}", input, error),
  };
}
```
You can also parse the building blocks of a triple (e.g. only a literal, or only a predicate, etc.). See the [tests](src/tests.rs) for more examples.

## API documentation
The full API documentation can be found [here](https://docs.rs/ntriple/).
