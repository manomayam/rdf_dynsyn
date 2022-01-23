use rio_turtle::TurtleError;
use rio_xml::RdfXmlError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum InnerParseError {
    Turtle(#[from] TurtleError),
    RdfXml(#[from] RdfXmlError),
}
