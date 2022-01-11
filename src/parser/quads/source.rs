use std::io::BufRead;

use rio_turtle::{NQuadsParser, TriGParser, TurtleError};
use sophia_api::{
    quad::{stream::QuadSource, streaming_mode::StreamedQuad},
    triple::stream::StreamResult,
};
use sophia_rio::parser::{ScopedRioSourceQuad, StrictRioSource};

use crate::parser::errors::{SomeSyntaxError, adapt_stream_result};


/// This quad-source proxies quad generation from underlying quad-source of allowed types
enum InnerQuadSource<R: BufRead> {
    FromNQuadsParser(StrictRioSource<NQuadsParser<R>, TurtleError>),

    FromTriGParser(StrictRioSource<TriGParser<R>, TurtleError>),
}

/// A [`QuadSource`](QuadSource) implementation, that abstract over other quad-sources that are specialized for some concrete syntaxes. Currently abstracts over quad-sources from [`NQuadsParser`](sophia_turtle::parser::nq::NQuadsParser) and [`TriGParser`](sophia_turtle::parser::trig::TriGParser).
pub struct SomeQuadSource<R: BufRead>(InnerQuadSource<R>);

impl<R: BufRead> From<StrictRioSource<NQuadsParser<R>, TurtleError>> for SomeQuadSource<R> {
    fn from(s: StrictRioSource<NQuadsParser<R>, TurtleError>) -> Self {
        Self(InnerQuadSource::FromNQuadsParser(s))
    }
}

impl<R: BufRead> From<StrictRioSource<TriGParser<R>, TurtleError>> for SomeQuadSource<R> {
    fn from(s: StrictRioSource<TriGParser<R>, TurtleError>) -> Self {
        Self(InnerQuadSource::FromTriGParser(s))
    }
}

impl<R: BufRead> QuadSource for SomeQuadSource<R> {
    type Error = SomeSyntaxError;

    type Quad = ScopedRioSourceQuad;

    fn try_for_some_quad<F, E>(&mut self, f: &mut F) -> StreamResult<bool, Self::Error, E>
    where
        F: FnMut(StreamedQuad<Self::Quad>) -> Result<(), E>,
        E: std::error::Error,
    {
        match &mut self.0 {
            InnerQuadSource::FromNQuadsParser(s) => adapt_stream_result(s.try_for_some_quad(f)),
            InnerQuadSource::FromTriGParser(s) => adapt_stream_result(s.try_for_some_quad(f)),
        }
    }
}
