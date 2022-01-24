use rio_turtle::TurtleError;
use rio_xml::RdfXmlError;
use sophia_api::triple::stream::{StreamError, StreamResult};

use super::_inner::errors::InnerParseError;
use crate::syntax::Syntax;

#[derive(Debug, thiserror::Error)]
#[error("Un supported syntax: {0}")]
pub struct UnKnownSyntaxError(pub Syntax);

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
/// An error that abstracts over other syntax parsing errors. Currently it can be constructed from [`TurtleError`](TurtleError), and [`RdfXmlError`](RdfXmlError)
pub struct DynSynParseError(InnerParseError);

impl From<TurtleError> for DynSynParseError {
    fn from(e: TurtleError) -> Self {
        Self(e.into())
    }
}

impl From<RdfXmlError> for DynSynParseError {
    fn from(e: RdfXmlError) -> Self {
        Self(e.into())
    }
}

pub type SomeStreamError<SinkErr> = StreamError<DynSynParseError, SinkErr>;

/// This function adapts StreamError by marshalling it's SourceError variant from known types to [`ParseError` ]type
pub fn adapt_quads_stream_error<SourceErr, SinkErr>(
    e: StreamError<SourceErr, SinkErr>,
) -> SomeStreamError<SinkErr>
where
    SourceErr: Into<DynSynParseError> + std::error::Error,
    SinkErr: std::error::Error,
{
    match e {
        StreamError::SourceError(ev) => StreamError::SourceError(ev.into()),
        StreamError::SinkError(ev) => StreamError::SinkError(ev),
    }
}

pub type DynSynStreamResult<T, SinkErr> = StreamResult<T, DynSynParseError, SinkErr>;

pub fn adapt_stream_result<T, SourceErr, SinkErr>(
    r: StreamResult<T, SourceErr, SinkErr>,
) -> DynSynStreamResult<T, SinkErr>
where
    SourceErr: Into<DynSynParseError> + std::error::Error,
    SinkErr: std::error::Error,
{
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(adapt_quads_stream_error(e)),
    }
}
