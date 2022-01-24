use sophia_turtle::parser::{
    nq::NQuadsParser, nt::NTriplesParser, trig::TriGParser, turtle::TurtleParser,
};
use sophia_xml::parser::RdfXmlParser;

use crate::syntax::{self, Syntax};

use super::errors::UnKnownSyntaxError;

pub mod source;

pub mod errors;

/// This is a sum-type that wraps around different rdf-syntax-parsers from sophia.
#[derive(Debug)]
pub enum InnerParser {
    NQuads(NQuadsParser),
    TriG(TriGParser),
    NTriples(NTriplesParser),
    Turtle(TurtleParser),
    RdfXml(RdfXmlParser),
}

impl From<NQuadsParser> for InnerParser {
    fn from(p: NQuadsParser) -> Self {
        Self::NQuads(p)
    }
}

impl From<TriGParser> for InnerParser {
    fn from(p: TriGParser) -> Self {
        Self::TriG(p)
    }
}

impl From<NTriplesParser> for InnerParser {
    fn from(p: NTriplesParser) -> Self {
        Self::NTriples(p)
    }
}

impl From<TurtleParser> for InnerParser {
    fn from(p: TurtleParser) -> Self {
        Self::Turtle(p)
    }
}

impl From<RdfXmlParser> for InnerParser {
    fn from(p: RdfXmlParser) -> Self {
        Self::RdfXml(p)
    }
}

impl InnerParser {
    /// Try to create a sum-parser for given syntax.
    /// 
    /// #Errors
    /// throws [1UnKnownSyntaxError] if syntax is unknown/un-supported
    pub fn try_new(syntax_: Syntax, base_iri: Option<String>) -> Result<Self, UnKnownSyntaxError> {
        match syntax_ {
            syntax::N_QUADS => Ok(NQuadsParser {}.into()),
            syntax::N_TRIPLES => Ok(NTriplesParser {}.into()),
            syntax::RDF_XML => Ok(RdfXmlParser { base: base_iri }.into()),
            syntax::TRIG => Ok(TriGParser { base: base_iri }.into()),
            syntax::TURTLE => Ok(TurtleParser { base: base_iri }.into()),
            _ => Err(UnKnownSyntaxError(syntax_)),
        }
    }
}
