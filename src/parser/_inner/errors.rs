use rio_turtle::TurtleError;
use rio_xml::RdfXmlError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum InnerParseError {
    TurtleError(#[from] TurtleError),
    RdfXmlError(#[from] RdfXmlError),
}
