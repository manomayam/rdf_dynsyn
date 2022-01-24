use std::{error::Error, io::BufRead};

use rio_api::parser::{QuadsParser, TriplesParser};
use sophia_api::{
    quad::{
        self,
        stream::QuadSource,
        streaming_mode::{ByValue, StreamedQuad},
        Quad,
    },
    term::{CopiableTerm, CopyTerm, TTerm},
    triple::{
        stream::{StreamResult, TripleSource},
        Triple,
    },
};
use sophia_rio::parser::StrictRioSource;

use crate::parser::{
    _inner::source::InnerStatementSource,
    errors::{adapt_stream_result, DynSynParseError},
};

pub type TupleQuad<T> = ([T; 3], Option<T>);

/// A [`QuadSource`], that adapts from another underlying quad-source/triple-source that can be of any supported types. Currently this implementation can adapt from quad_sources/triple-sources that are returned by major sophia parsers.
///
/// If underlying statement source is a quad-source, then it will emit equivalent quads.
///
/// If underlying statement source is a triple-source, then it will emit quads corresponding to each triple, with graph_name term set to configured `triple_source_graph_iri`  field value, and remaining terms  being equivalent to those of triple.
pub struct DynSynQuadSource<T: CopyTerm + TTerm, R: BufRead> {
    inner_source: InnerStatementSource<R>,
    triple_source_graph_iri: Option<T>,
}

impl<T: CopyTerm + TTerm + Clone, R: BufRead> DynSynQuadSource<T, R> {
    /// Call `f` for at least one adapted-quad (if any) that is adapted from underlying rio quad source.
    ///
    /// Return false if no more quads can be adapted from underlying source.
    ///
    /// If underlying fallible quad-source returns a parse error, then that error will be wrapped in enum [`DynSynParseError`] as an appropriate variant.
    fn try_for_some_quad_adapted_from_rio_quad_source<Parser, PErr, SinkErr, F>(
        // underlying quad source
        qs: &mut StrictRioSource<Parser, PErr>,
        mut f: F,
    ) -> StreamResult<bool, DynSynParseError, SinkErr>
    where
        Parser: QuadsParser<Error = PErr>,
        PErr: Error + 'static + Into<DynSynParseError>,
        SinkErr: Error,
        F: FnMut(StreamedQuad<ByValue<TupleQuad<T>>>) -> Result<(), SinkErr>,
    {
        adapt_stream_result(qs.try_for_some_quad(&mut |q| {
            let tq: TupleQuad<T> = (
                [q.s().copied(), q.p().copied(), q.o().copied()],
                q.g().and_then(|gv| Some(gv.copied())),
            );
            f(StreamedQuad::by_value(tq))
        }))
    }

    /// Call `f` for at least one adapted-quad (if any) that is adapted from underlying rio triple source.
    ///
    /// Return false if no more quads can be adapted from underlying source.
    ///
    /// If underlying fallible triple-source returns a parse error, then that error will be wrapped in enum [`DynSynParseError`] as an appropriate variant.
    ///
    /// # Triple to Quad adaptation:
    ///  Each triple from underlying triple-source will be adapted into a quad, with graph_name term set to configured `triple_source_graph_iri`  param value, and remaining terms  being equivalent to those of triple.
    fn try_for_some_quad_adapted_from_rio_triple_source<Parser, PErr, SinkErr, F>(
        ts: &mut StrictRioSource<Parser, PErr>,
        mut f: F,
        triple_source_graph_iri: &Option<T>,
    ) -> StreamResult<bool, DynSynParseError, SinkErr>
    where
        Parser: TriplesParser<Error = PErr>,
        PErr: Error + 'static + Into<DynSynParseError>,
        SinkErr: Error,
        F: FnMut(StreamedQuad<ByValue<TupleQuad<T>>>) -> Result<(), SinkErr>,
    {
        adapt_stream_result(ts.try_for_some_triple(&mut |t| {
            let tq: TupleQuad<T> = (
                [t.s().copied(), t.p().copied(), t.o().copied()],
                triple_source_graph_iri.clone().and_then(|gv| Some(gv)),
            );
            f(StreamedQuad::by_value(tq))
        }))
    }

    pub(crate) fn new_for(
        inner_source: InnerStatementSource<R>,
        triple_source_graph_iri: Option<T>,
    ) -> Self {
        Self {
            inner_source,
            triple_source_graph_iri,
        }
    }
}

impl<T, R> quad::stream::QuadSource for DynSynQuadSource<T, R>
where
    T: CopyTerm + TTerm + Clone,
    R: BufRead,
{
    type Error = DynSynParseError;

    type Quad = ByValue<TupleQuad<T>>;

    fn try_for_some_quad<F, E>(&mut self, f: &mut F) -> StreamResult<bool, Self::Error, E>
    where
        F: FnMut(StreamedQuad<Self::Quad>) -> Result<(), E>,
        E: std::error::Error,
    {
        match &mut self.inner_source {
            InnerStatementSource::FNQuads(qs) => {
                Self::try_for_some_quad_adapted_from_rio_quad_source(qs, f)
            }

            InnerStatementSource::FTriG(qs) => {
                Self::try_for_some_quad_adapted_from_rio_quad_source(qs, f)
            }

            InnerStatementSource::FNTriples(ts) => {
                Self::try_for_some_quad_adapted_from_rio_triple_source(
                    ts,
                    f,
                    &self.triple_source_graph_iri,
                )
            }

            InnerStatementSource::FTurtle(ts) => {
                Self::try_for_some_quad_adapted_from_rio_triple_source(
                    ts,
                    f,
                    &self.triple_source_graph_iri,
                )
            }

            InnerStatementSource::FRdfXml(ts) => {
                Self::try_for_some_quad_adapted_from_rio_triple_source(
                    ts,
                    f,
                    &self.triple_source_graph_iri,
                )
            }
        }
    }
}
