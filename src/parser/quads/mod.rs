use std::io::BufRead;

use sophia_api::{
    parser::{QuadParser, TripleParser},
    term::{CopyTerm, TTerm},
};

use crate::syntax::Syntax;

use self::source::SomeHowQuadSource;

use super::{_inner::InnerParser, errors::UnSupportedSyntaxError};

pub mod source;

pub struct SomeHowQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    inner_parser: InnerParser,
    triple_source_graph_iri: Option<T>,
}

impl<T> SomeHowQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    pub fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
        triple_source_graph_iri: Option<T>,
    ) -> Result<Self, UnSupportedSyntaxError> {
        let inner_parser = InnerParser::try_new(syntax_, base_iri)?;
        Ok(Self {
            inner_parser,
            triple_source_graph_iri,
        })
    }
}

impl<T, R> QuadParser<R> for SomeHowQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
    R: BufRead,
{
    type Source = SomeHowQuadSource<T, R>;

    fn parse(&self, data: R) -> Self::Source {
        let tsg_iri = self.triple_source_graph_iri.clone();
        // TODO may be abstract over literal repetition
        match &self.inner_parser {
            InnerParser::NQuadsParser(p) => {
                SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri)
            }
            InnerParser::TriGParser(p) => SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::NTriplesParser(p) => {
                SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri)
            }
            InnerParser::TurtleParser(p) => {
                SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri)
            }
            InnerParser::RdfXmParser(p) => {
                SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri)
            }
        }
    }
}
