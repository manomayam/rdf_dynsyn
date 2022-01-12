use sophia_turtle::parser::{
    nq::NQuadsParser, nt::NTriplesParser, trig::TriGParser, turtle::TurtleParser,
};
use sophia_xml::parser::RdfXmlParser;

use crate::syntax::{self, Syntax};

use super::errors::UnSupportedSyntaxError;

pub mod source;

pub mod errors;

pub enum InnerParser {
    NQuadsParser(NQuadsParser),
    TriGParser(TriGParser),
    NTriplesParser(NTriplesParser),
    TurtleParser(TurtleParser),
    RdfXmParser(RdfXmlParser),
}

impl From<NQuadsParser> for InnerParser {
    fn from(p: NQuadsParser) -> Self {
        Self::NQuadsParser(p)
    }
}

impl From<TriGParser> for InnerParser {
    fn from(p: TriGParser) -> Self {
        Self::TriGParser(p)
    }
}

impl From<NTriplesParser> for InnerParser {
    fn from(p: NTriplesParser) -> Self {
        Self::NTriplesParser(p)
    }
}

impl From<TurtleParser> for InnerParser {
    fn from(p: TurtleParser) -> Self {
        Self::TurtleParser(p)
    }
}

impl From<RdfXmlParser> for InnerParser {
    fn from(p: RdfXmlParser) -> Self {
        Self::RdfXmParser(p)
    }
}

impl InnerParser {
    pub fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
    ) -> Result<Self, UnSupportedSyntaxError> {
        match syntax_ {
            syntax::N_QUADS => Ok(NQuadsParser {}.into()),
            syntax::N_TRIPLES => Ok(NTriplesParser {}.into()),
            syntax::RDF_XML => Ok(RdfXmlParser { base: base_iri }.into()),
            syntax::TRIG => Ok(TriGParser { base: base_iri }.into()),
            syntax::TURTLE => Ok(TurtleParser { base: base_iri }.into()),
            _ => Err(UnSupportedSyntaxError(syntax_)),
        }
    }
}
