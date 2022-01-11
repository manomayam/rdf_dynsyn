use std::io::BufRead;

use sophia_api::parser::QuadParser;
use sophia_turtle::parser::{nq, trig};

use crate::syntax::{self, Syntax};

use self::source::SomeQuadSource;

pub mod source;

#[derive(Debug, thiserror::Error)]
#[error("Un supported quad syntax: {0}")]
pub struct UnSupportedQuadSyntaxError(Syntax);

enum InnerQuadParser {
    NQuadsParser(nq::NQuadsParser),
    TriGParser(trig::TriGParser),
}

/// A parser that can parse quads from documents in concrete syntax, with which this parser is instantiated at runtime. Currently it supports NQuads and TriG syntaxes.
pub struct SomeSyntaxQuadParser(InnerQuadParser);

impl SomeSyntaxQuadParser {
    /// Try to create an instance of the parser for given syntax. returns [`UnSupportedQuadSyntaxError`](UnSupportedQuadSyntaxError) if syntax is not a supported syntax for quads parsing
    pub fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
    ) -> Result<Self, UnSupportedQuadSyntaxError> {
        if syntax_ == syntax::N_QUADS {
            Ok(Self(InnerQuadParser::NQuadsParser(nq::NQuadsParser {})))
        } else if syntax_ == syntax::TRIG {
            Ok(Self(InnerQuadParser::TriGParser(trig::TriGParser {
                base: base_iri,
            })))
        } else {
            Err(UnSupportedQuadSyntaxError(syntax_))
        }
    }
}

impl<R: BufRead> QuadParser<R> for SomeSyntaxQuadParser {
    type Source = SomeQuadSource<R>;

    fn parse(&self, data: R) -> Self::Source {
        match &self.0 {
            InnerQuadParser::NQuadsParser(p) => p.parse(data).into(),
            InnerQuadParser::TriGParser(p) => p.parse(data).into(),
        }
    }
}
