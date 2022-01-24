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

pub struct DynSynQuadSource<T: CopyTerm + TTerm, R: BufRead> {
    inner_source: InnerStatementSource<R>,
    triple_source_graph_iri: Option<T>,
}

impl<T: CopyTerm + TTerm + Clone, R: BufRead> DynSynQuadSource<T, R> {
    fn map_from_rio_quad_source<Parser, PErr, SinkErr, F>(
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

    fn map_from_rio_triple_source<Parser, PErr, SinkErr, F>(
        ts: &mut StrictRioSource<Parser, PErr>,
        mut f: F,
        graph_iri: &Option<T>,
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
                graph_iri.clone().and_then(|gv| Some(gv)),
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
            InnerStatementSource::FNQuads(qs) => Self::map_from_rio_quad_source(qs, f),

            InnerStatementSource::FTriG(qs) => Self::map_from_rio_quad_source(qs, f),

            InnerStatementSource::FNTriples(ts) => {
                Self::map_from_rio_triple_source(ts, f, &self.triple_source_graph_iri)
            }

            InnerStatementSource::FTurtle(ts) => {
                Self::map_from_rio_triple_source(ts, f, &self.triple_source_graph_iri)
            }

            InnerStatementSource::FRdfXml(ts) => {
                Self::map_from_rio_triple_source(ts, f, &self.triple_source_graph_iri)
            }
        }
    }
}
