use std::io;

use sophia_api::serializer::TripleSerializer;
use sophia_turtle::serializer::{
    nt::{NtConfig, NtSerializer},
    turtle::{TurtleConfig, TurtleSerializer},
};
use sophia_xml::serializer::{RdfXmlConfig, RdfXmlSerializer};
use type_map::TypeMap;

use crate::{
    parser::errors::UnKnownSyntaxError,
    syntax::{self, Syntax},
};

use super::_inner::InnerTripleSerializer;

pub struct SomeHowTripleSerializer<W: io::Write> {
    inner_serializer: InnerTripleSerializer<W>,
}

impl<W: io::Write> SomeHowTripleSerializer<W> {
    pub(crate) fn new(inner_serializer: InnerTripleSerializer<W>) -> Self {
        Self { inner_serializer }
    }
}

impl<W: io::Write> TripleSerializer for SomeHowTripleSerializer<W> {
    type Error = io::Error;

    fn serialize_triples<TS>(
        &mut self,
        source: TS,
    ) -> sophia_api::triple::stream::StreamResult<&mut Self, TS::Error, Self::Error>
    where
        TS: sophia_api::triple::stream::TripleSource,
        Self: Sized,
    {
        match &mut self.inner_serializer {
            InnerTripleSerializer::NTriples(s) => match s.serialize_triples(source) {
                Ok(_) => Ok(self),
                Err(e) => Err(e),
            },
            InnerTripleSerializer::Turtle(s) => match s.serialize_triples(source) {
                Ok(_) => Ok(self),
                Err(e) => Err(e),
            },
            InnerTripleSerializer::RdfXml(s) => match s.serialize_triples(source) {
                Ok(_) => Ok(self),
                Err(e) => Err(e),
            },
        }
    }
}

pub struct SomeHowTripleSerializerFactory {
    serializer_config_map: TypeMap,
}

impl SomeHowTripleSerializerFactory {
    pub fn new(serializer_config_map: TypeMap) -> Self {
        Self {
            serializer_config_map,
        }
    }

    fn get_config<T: Clone + Default + 'static>(&self) -> T {
        self.serializer_config_map
            .get::<T>()
            .and_then(|c| Some(c.clone()))
            .unwrap_or(Default::default())
    }

    pub fn try_new_serializer<W: io::Write>(
        &self,
        syntax_: Syntax,
        write: W,
    ) -> Result<SomeHowTripleSerializer<W>, UnKnownSyntaxError> {
        match syntax_ {
            syntax::N_TRIPLES => Ok(SomeHowTripleSerializer::new(
                InnerTripleSerializer::NTriples(NtSerializer::new_with_config(
                    write,
                    self.get_config::<NtConfig>(),
                )),
            )),
            syntax::TURTLE => Ok(SomeHowTripleSerializer::new(InnerTripleSerializer::Turtle(
                TurtleSerializer::new_with_config(write, self.get_config::<TurtleConfig>()),
            ))),
            syntax::RDF_XML => Ok(SomeHowTripleSerializer::new(InnerTripleSerializer::RdfXml(
                RdfXmlSerializer::new_with_config(write, self.get_config::<RdfXmlConfig>()),
            ))),
            _ => Err(UnKnownSyntaxError(syntax_)),
        }
    }
}
