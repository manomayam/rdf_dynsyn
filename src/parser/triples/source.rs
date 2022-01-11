use std::io::BufRead;

use rio_turtle::{NTriplesParser, TurtleError, TurtleParser};
use rio_xml::{RdfXmlError, RdfXmlParser};
use sophia_api::triple::{
    stream::{StreamResult, TripleSource},
    streaming_mode::StreamedTriple,
};
use sophia_rio::parser::{ScopedRioSourceTriple, StrictRioSource};

use crate::parser::errors::{SomeSyntaxError, adapt_stream_result};


/// This triple-source proxies triple generation from underlying triple-source of allowed types
enum InnerTripleSource<R: BufRead> {
    FromNTriplesParser(StrictRioSource<NTriplesParser<R>, TurtleError>),

    FromTurtleParser(StrictRioSource<TurtleParser<R>, TurtleError>),

    FromRdfXmlParser(StrictRioSource<RdfXmlParser<R>, RdfXmlError>),
}

/// A [`TripleSource`](TripleSource) implementation, that abstract over other triple-sources that are specialized for some concrete syntaxes. Currently can abstract over triple-sources from [`NTriplesParser`], [`TurtleParser`].and [`RdfXmlParser`]
pub struct SomeTripleSource<R: BufRead>(InnerTripleSource<R>);

impl<R: BufRead> From<StrictRioSource<NTriplesParser<R>, TurtleError>> for SomeTripleSource<R> {
    fn from(s: StrictRioSource<NTriplesParser<R>, TurtleError>) -> Self {
        Self(InnerTripleSource::FromNTriplesParser(s))
    }
}

impl<R: BufRead> From<StrictRioSource<TurtleParser<R>, TurtleError>> for SomeTripleSource<R> {
    fn from(s: StrictRioSource<TurtleParser<R>, TurtleError>) -> Self {
        Self(InnerTripleSource::FromTurtleParser(s))
    }
}

impl<R: BufRead> From<StrictRioSource<RdfXmlParser<R>, RdfXmlError>> for SomeTripleSource<R> {
    fn from(s: StrictRioSource<RdfXmlParser<R>, RdfXmlError>) -> Self {
        Self(InnerTripleSource::FromRdfXmlParser(s))
    }
}

impl<R: BufRead> TripleSource for SomeTripleSource<R> {
    type Error = SomeSyntaxError;
    type Triple = ScopedRioSourceTriple;

    fn try_for_some_triple<F, E>(&mut self, f: &mut F) -> StreamResult<bool, Self::Error, E>
    where
        F: FnMut(StreamedTriple<Self::Triple>) -> Result<(), E>,
        E: std::error::Error,
    {
        match &mut self.0 {
            InnerTripleSource::FromNTriplesParser(s) => {
                adapt_stream_result(s.try_for_some_triple(f))
            }
            InnerTripleSource::FromTurtleParser(s) => adapt_stream_result(s.try_for_some_triple(f)),
            InnerTripleSource::FromRdfXmlParser(s) => adapt_stream_result(s.try_for_some_triple(f)),
        }
    }
}
