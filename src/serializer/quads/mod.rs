use std::io;

use sophia_api::serializer::QuadSerializer;
use sophia_turtle::serializer::{
    nq::{NqConfig, NqSerializer},
    trig::{TrigConfig, TrigSerializer},
};
use type_map::TypeMap;

use crate::{
    parser::errors::UnKnownSyntaxError,
    syntax::{self, Syntax},
};

use super::_inner::InnerQuadSerializer;

pub struct SomeHowQuadSerializer<W: io::Write> {
    inner_serializer: InnerQuadSerializer<W>,
}

impl<W: io::Write> SomeHowQuadSerializer<W> {
    pub(crate) fn new(inner_serializer: InnerQuadSerializer<W>) -> Self {
        Self { inner_serializer }
    }
}

impl<W: io::Write> QuadSerializer for SomeHowQuadSerializer<W> {
    type Error = io::Error;

    fn serialize_quads<QS>(
        &mut self,
        source: QS,
    ) -> sophia_api::triple::stream::StreamResult<&mut Self, QS::Error, Self::Error>
    where
        QS: sophia_api::quad::stream::QuadSource,
        Self: Sized,
    {
        match &mut self.inner_serializer {
            InnerQuadSerializer::NQuads(s) => match s.serialize_quads(source) {
                Ok(_) => Ok(self),
                Err(e) => Err(e),
            },
            InnerQuadSerializer::Trig(s) => match s.serialize_quads(source) {
                Ok(_) => Ok(self),
                Err(e) => Err(e),
            },
        }
    }
}

pub struct SomeHowQuadSerializerFactory {
    serializer_config_map: TypeMap,
}

impl SomeHowQuadSerializerFactory {
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
    ) -> Result<SomeHowQuadSerializer<W>, UnKnownSyntaxError> {
        match syntax_ {
            syntax::N_QUADS => Ok(SomeHowQuadSerializer::new(InnerQuadSerializer::NQuads(
                NqSerializer::new_with_config(write, self.get_config::<NqConfig>()),
            ))),
            syntax::TRIG => Ok(SomeHowQuadSerializer::new(InnerQuadSerializer::Trig(
                TrigSerializer::new_with_config(write, self.get_config::<TrigConfig>()),
            ))),
            _ => Err(UnKnownSyntaxError(syntax_)),
        }
    }
}
