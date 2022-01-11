use std::io::BufRead;

use sophia_api::parser::TripleParser;
use sophia_turtle::parser::{nt, turtle};
use sophia_xml::parser::RdfXmlParser;

use crate::syntax::{Syntax, self};

use self::source::SomeTripleSource;

pub mod source;

#[derive(Debug, thiserror::Error)]
#[error("Un supported triple syntax: {0}")]
pub struct UnSupportedTripleSyntaxError(Syntax);

enum InnerTripleParser {
    RdfXmlParser(RdfXmlParser),
    TurtleParser(turtle::TurtleParser),
    NTripleParser(nt::NTriplesParser),
}

/// A parser that can parse quads from documents in concrete syntax, with which this parser is instantiated at runtime Currently it supports NQuads and TriG syntaxes.
pub struct SomeSyntaxTripleParser(InnerTripleParser);

impl SomeSyntaxTripleParser {
    /// Try to create an instance of the parser for given syntax. returns [`UnSupportedTripleSyntaxError`](UnSupportedTripleSyntaxError) if syntax is not a supported syntax for triple parsing
    pub fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
    ) -> Result<Self, UnSupportedTripleSyntaxError> {
        if syntax_ == syntax::N_TRIPLES {
            Ok(Self(InnerTripleParser::NTripleParser(
                nt::NTriplesParser {},
            )))
        } else if syntax_ == syntax::TURTLE {
            Ok(Self(InnerTripleParser::TurtleParser(
                turtle::TurtleParser { base: base_iri },
            )))
        } else if syntax_ == syntax::RDF_XML {
            Ok(Self(InnerTripleParser::RdfXmlParser(RdfXmlParser {
                base: base_iri,
            })))
        } else {
            Err(UnSupportedTripleSyntaxError(syntax_))
        }
    }
}

impl<R: BufRead> TripleParser<R> for SomeSyntaxTripleParser {
    type Source = SomeTripleSource<R>;

    fn parse(&self, data: R) -> Self::Source {
        match &self.0 {
            InnerTripleParser::RdfXmlParser(p) => p.parse(data).into(),
            InnerTripleParser::TurtleParser(p) => p.parse(data).into(),
            InnerTripleParser::NTripleParser(p) => p.parse(data).into(),
        }
    }
}
