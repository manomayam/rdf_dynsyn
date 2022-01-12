use std::io::BufRead;

use sophia_turtle::parser::{
    nq::NQuadsParser, nt::NTriplesParser, trig::TriGParser, turtle::TurtleParser,
};
use sophia_xml::parser::RdfXmlParser;

use crate::syntax::{self, Syntax};

pub mod errors;
pub mod quads;

#[derive(Debug, thiserror::Error)]
#[error("Un supported quad syntax: {0}")]
pub struct UnSupportedQuadSyntaxError(Syntax);

enum InnerParser {
    NQuadsParser(NQuadsParser),
    TriGParser(TriGParser),
    NTriplesParser(NTriplesParser),
    TurtleParser(TurtleParser),
    RdfXmParser(RdfXmlParser),
}

/// A parser that can parse quads from documents in concrete syntax, with which this parser is instantiated at runtime. Currently it supports NQuads and TriG syntaxes.
pub struct SomeSyntaxQuadParser {
    inner_parser: InnerParser,
    // quad_map_fn:
}

/*
impl SomeSyntaxQuadParser {
    /// Try to create an instance of the parser for given syntax. returns [`UnSupportedQuadSyntaxError`](UnSupportedQuadSyntaxError) if syntax is not a supported syntax for quads parsing
    pub fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
    ) -> Result<Self, UnSupportedQuadSyntaxError> {
        if syntax_ == syntax::N_QUADS {
            Ok(Self(InnerParser::NQuadsParser(nq::NQuadsParser {})))
        } else if syntax_ == syntax::TRIG {
            Ok(Self(InnerParser::TriGParser(trig::TriGParser {
                base: base_iri,
            })))
        } else {
            Err(UnSupportedQuadSyntaxError(syntax_))
        }
    }
}

impl<R: BufRead> QuadParser<R> for SomeSyntaxQuadParser {
    type Source = SomeHowQuadSource<R>;

    fn parse(&self, data: R) -> Self::Source {
        match &self.0 {
            InnerParser::NQuadsParser(p) => p.parse(data).into(),
            InnerParser::TriGParser(p) => p.parse(data).into(),
        }
    }
}
*/
