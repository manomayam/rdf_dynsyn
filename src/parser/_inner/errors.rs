//! This module defines sum-types over errors that arise from underlying parsers

use rio_turtle::TurtleError;
use rio_xml::RdfXmlError;


/// This is a sum-type that wraps around different rdf-syntax-parse-errors, that arise from different sophia parsers.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum InnerParseError {
    Turtle(#[from] TurtleError),
    RdfXml(#[from] RdfXmlError),
}
