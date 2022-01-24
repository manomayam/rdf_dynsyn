use std::{fmt::Debug, io};

use sophia_turtle::serializer::{
    nq::NqSerializer, nt::NtSerializer, trig::TrigSerializer, turtle::TurtleSerializer,
};
use sophia_xml::serializer::RdfXmlSerializer;

pub mod errors;

pub(crate) enum InnerQuadSerializer<W: io::Write> {
    NQuads(NqSerializer<W>),
    Trig(TrigSerializer<W>),
}

impl<W: io::Write> Debug for InnerQuadSerializer<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NQuads(_) => f.debug_tuple("NQuads").finish(),
            Self::Trig(_) => f.debug_tuple("Trig").finish(),
        }
    }
}

pub(crate) enum InnerTripleSerializer<W: io::Write> {
    NTriples(NtSerializer<W>),
    Turtle(TurtleSerializer<W>),
    RdfXml(RdfXmlSerializer<W>),
}

impl<W: io::Write> Debug for InnerTripleSerializer<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NTriples(_) => f.debug_tuple("NTriples").finish(),
            Self::Turtle(_) => f.debug_tuple("Turtle").finish(),
            Self::RdfXml(_) => f.debug_tuple("RdfXml").finish(),
        }
    }
}
