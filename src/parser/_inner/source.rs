//! This module defines sum-types over streaming sources that are produced from underlying parsers

use std::io::BufRead;

use rio_turtle::{NQuadsParser, NTriplesParser, TriGParser, TurtleError, TurtleParser};
use rio_xml::{RdfXmlError, RdfXmlParser};
use sophia_rio::parser::StrictRioSource;

/// This is a sum-type that wraps around different rdf-streaming-sources (currently those, which implements  either [`QuadSource`](sophia_api::quad::stream::QuadSource) or [`TripleSource`](sophia_api::triple::stream::TripleSource) trait), that are normally produced by different sophia parsers.
pub enum InnerStatementSource<R: BufRead> {
    FNQuads(StrictRioSource<NQuadsParser<R>, TurtleError>),
    FTriG(StrictRioSource<TriGParser<R>, TurtleError>),
    FNTriples(StrictRioSource<NTriplesParser<R>, TurtleError>),
    FTurtle(StrictRioSource<TurtleParser<R>, TurtleError>),
    FRdfXml(StrictRioSource<RdfXmlParser<R>, RdfXmlError>),
}

impl<R: BufRead> From<StrictRioSource<NQuadsParser<R>, TurtleError>> for InnerStatementSource<R> {
    fn from(qs: StrictRioSource<NQuadsParser<R>, TurtleError>) -> Self {
        Self::FNQuads(qs)
    }
}

impl<R: BufRead> From<StrictRioSource<TriGParser<R>, TurtleError>> for InnerStatementSource<R> {
    fn from(qs: StrictRioSource<TriGParser<R>, TurtleError>) -> Self {
        Self::FTriG(qs)
    }
}
impl<R: BufRead> From<StrictRioSource<NTriplesParser<R>, TurtleError>> for InnerStatementSource<R> {
    fn from(ts: StrictRioSource<NTriplesParser<R>, TurtleError>) -> Self {
        Self::FNTriples(ts)
    }
}
impl<R: BufRead> From<StrictRioSource<TurtleParser<R>, TurtleError>> for InnerStatementSource<R> {
    fn from(ts: StrictRioSource<TurtleParser<R>, TurtleError>) -> Self {
        Self::FTurtle(ts)
    }
}

impl<R: BufRead> From<StrictRioSource<RdfXmlParser<R>, RdfXmlError>> for InnerStatementSource<R> {
    fn from(ts: StrictRioSource<RdfXmlParser<R>, RdfXmlError>) -> Self {
        Self::FRdfXml(ts)
    }
}
