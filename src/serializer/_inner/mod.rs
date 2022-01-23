use std::io;

use sophia_turtle::serializer::{nq::NqSerializer, nt::NtSerializer, turtle::TurtleSerializer, trig::TrigSerializer};
use sophia_xml::serializer::RdfXmlSerializer;

pub mod errors;

pub(crate) enum InnerQuadSerializer<W: io::Write> {
    NQuads(NqSerializer<W>),
    Trig(TrigSerializer<W>),
}

pub(crate) enum InnerTripleSerializer<W: io::Write> {
    NTriples(NtSerializer<W>),
    Turtle(TurtleSerializer<W>),
    RdfXml(RdfXmlSerializer<W>),
}

