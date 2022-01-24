use std::{error::Error, io::BufRead};

use rio_api::parser::{QuadsParser, TriplesParser};
use sophia_api::{
    quad::{stream::QuadSource, Quad},
    term::{term_eq, CopiableTerm, CopyTerm, TTerm},
    triple::{
        self,
        stream::{StreamResult, TripleSource},
        streaming_mode::{ByValue, StreamedTriple},
        Triple,
    },
};
use sophia_rio::parser::StrictRioSource;

use crate::parser::{
    _inner::source::InnerStatementSource,
    errors::{adapt_stream_result, DynSynParseError},
};

pub type SliceTriple<T> = [T; 3];

/// A [`TripleSource`], that adapts from another underlying triple-source/quad-source that can be of any supported types. Currently this implementation can adapt from triple_sources/quad-sources that are returned by major sophia parsers.
///
/// If underlying statement source is a triple-source, then it will emit equivalent triples.
///
/// If underlying statement source is a quad-source, then it will emit triples corresponding to each quad that have  graph_name term set to configured `quad_source_adapted_graph_iri`  field value. quads that have different graph_name term will be ignored in such case.
pub struct DynSynTripleSource<T: CopyTerm + TTerm, R: BufRead> {
    inner_source: InnerStatementSource<R>,
    quad_source_adapted_graph_iri: Option<T>,
}

impl<T: CopyTerm + TTerm + Clone, R: BufRead> DynSynTripleSource<T, R> {
    /// Call `f` for at least one adapted-triple (if any) that is adapted from underlying rio quad source.
    ///
    /// Return false if no more triple can be adapted from underlying source.
    ///
    /// If underlying fallible quad-source returns a parse error, then that error will be wrapped in enum [`DynSynParseError`] as an appropriate variant.
    ///
    /// # Quad to Triple adaptation:
    ///  Each quad from underlying quad-source, which has it's graph_name term same as `quad_source_adapted_graph_iri`  will be adapted into a triple. Quads with any other graph_name term will be ignored.
    fn try_for_some_triple_adapted_from_rio_quad_source<Parser, PErr, SinkErr, F>(
        qs: &mut StrictRioSource<Parser, PErr>,
        mut f: F,
        quad_source_adapted_graph_iri: &Option<T>,
    ) -> StreamResult<bool, DynSynParseError, SinkErr>
    where
        Parser: QuadsParser<Error = PErr>,
        PErr: Error + 'static + Into<DynSynParseError>,
        SinkErr: Error,
        F: FnMut(StreamedTriple<ByValue<SliceTriple<T>>>) -> Result<(), SinkErr>,
    {
        adapt_stream_result(qs.try_for_some_quad(&mut |q| {
            let in_graph = match (q.g(), quad_source_adapted_graph_iri) {
                (Some(a), Some(b)) => term_eq(a, b),
                (None, None) => true,
                _ => false,
            };
            if !in_graph {
                return Ok(());
            }
            let tq: SliceTriple<T> = [q.s().copied(), q.p().copied(), q.o().copied()];
            f(StreamedTriple::by_value(tq))
        }))
    }
    /// Call `f` for at least one adapted-triple (if any) that is adapted from underlying rio triple source.
    ///
    /// Return false if no more triples can be adapted from underlying source.
    ///
    /// If underlying fallible triple-source returns a parse error, then that error will be wrapped in enum [`DynSynParseError`] as an appropriate variant.
    fn try_for_some_triple_adapted_from_rio_triple_source<Parser, PErr, SinkErr, F>(
        ts: &mut StrictRioSource<Parser, PErr>,
        mut f: F,
    ) -> StreamResult<bool, DynSynParseError, SinkErr>
    where
        Parser: TriplesParser<Error = PErr>,
        PErr: Error + 'static + Into<DynSynParseError>,
        SinkErr: Error,
        F: FnMut(StreamedTriple<ByValue<SliceTriple<T>>>) -> Result<(), SinkErr>,
    {
        adapt_stream_result(ts.try_for_some_triple(&mut |t| {
            let tq: SliceTriple<T> = [t.s().copied(), t.p().copied(), t.o().copied()];
            f(StreamedTriple::by_value(tq))
        }))
    }

    pub(crate) fn new_for(
        inner_source: InnerStatementSource<R>,
        quad_source_virtual_default_graph_iri: Option<T>,
    ) -> Self {
        Self {
            inner_source,
            quad_source_adapted_graph_iri: quad_source_virtual_default_graph_iri,
        }
    }
}

impl<T, R> triple::stream::TripleSource for DynSynTripleSource<T, R>
where
    T: CopyTerm + TTerm + Clone,
    R: BufRead,
{
    type Error = DynSynParseError;

    type Triple = ByValue<SliceTriple<T>>;

    fn try_for_some_triple<F, E>(&mut self, f: &mut F) -> StreamResult<bool, Self::Error, E>
    where
        F: FnMut(StreamedTriple<Self::Triple>) -> Result<(), E>,
        E: Error,
    {
        match &mut self.inner_source {
            InnerStatementSource::FNQuads(qs) => {
                Self::try_for_some_triple_adapted_from_rio_quad_source(
                    qs,
                    f,
                    &self.quad_source_adapted_graph_iri,
                )
            }

            InnerStatementSource::FTriG(qs) => {
                Self::try_for_some_triple_adapted_from_rio_quad_source(
                    qs,
                    f,
                    &self.quad_source_adapted_graph_iri,
                )
            }

            InnerStatementSource::FNTriples(ts) => {
                Self::try_for_some_triple_adapted_from_rio_triple_source(ts, f)
            }

            InnerStatementSource::FTurtle(ts) => {
                Self::try_for_some_triple_adapted_from_rio_triple_source(ts, f)
            }

            InnerStatementSource::FRdfXml(ts) => {
                Self::try_for_some_triple_adapted_from_rio_triple_source(ts, f)
            }
        }
    }
}
