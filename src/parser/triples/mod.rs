use std::io::BufRead;

use sophia_api::{
    parser::{QuadParser, TripleParser},
    term::{CopyTerm, TTerm},
};

use crate::syntax::Syntax;

use self::source::SomeHowTripleSource;

use super::{_inner::InnerParser, errors::UnSupportedSyntaxError};

pub mod source;

pub struct SomeHowTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    inner_parser: InnerParser,
    quad_source_virtual_default_graph_iri: Option<T>,
}

impl<T> SomeHowTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    pub fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
        quad_source_virtual_default_graph_iri: Option<T>,
    ) -> Result<Self, UnSupportedSyntaxError> {
        let inner_parser = InnerParser::try_new(syntax_, base_iri)?;
        Ok(Self {
            inner_parser,
            quad_source_virtual_default_graph_iri,
        })
    }
}

impl<T, R> TripleParser<R> for SomeHowTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
    R: BufRead,
{
    type Source = SomeHowTripleSource<T, R>;

    fn parse(&self, data: R) -> Self::Source {
        let tsg_iri = self.quad_source_virtual_default_graph_iri.clone();
        // TODO may be abstract over literal repetition
        match &self.inner_parser {
            InnerParser::NQuadsParser(p) => {
                SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri)
            }
            InnerParser::TriGParser(p) => SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::NTriplesParser(p) => {
                SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri)
            }
            InnerParser::TurtleParser(p) => {
                SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri)
            }
            InnerParser::RdfXmParser(p) => {
                SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri)
            }
        }
    }
}
