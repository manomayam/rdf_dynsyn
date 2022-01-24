use std::io;

use sophia_api::serializer::{QuadSerializer, Stringifier};
use sophia_turtle::serializer::{
    nq::{NqConfig, NqSerializer},
    trig::{TrigConfig, TrigSerializer},
};
use type_map::concurrent::TypeMap;

use crate::{
    parser::errors::UnKnownSyntaxError,
    syntax::{self, Syntax},
};

use super::_inner::InnerQuadSerializer;

#[derive(Debug)]
pub struct DynSynQuadSerializer<W: io::Write> {
    inner_serializer: InnerQuadSerializer<W>, // NOTE can be a trait object. serializers seems amenable to be trait objects unlike parsers and sources
}

impl<W: io::Write> DynSynQuadSerializer<W> {
    pub(crate) fn new(inner_serializer: InnerQuadSerializer<W>) -> Self {
        Self { inner_serializer }
    }
}

impl<W: io::Write> QuadSerializer for DynSynQuadSerializer<W> {
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

impl Stringifier for DynSynQuadSerializer<Vec<u8>> {
    fn as_utf8(&self) -> &[u8] {
        match &self.inner_serializer {
            InnerQuadSerializer::NQuads(s) => s.as_utf8(),
            InnerQuadSerializer::Trig(s) => s.as_utf8(),
        }
    }
}

pub struct DynSynQuadSerializerFactory {
    serializer_config_map: TypeMap,
}

impl DynSynQuadSerializerFactory {
    pub fn new(serializer_config_map: TypeMap) -> Self {
        Self {
            serializer_config_map,
        }
    }

    pub fn get_config<T: Clone + Default + 'static>(&self) -> T {
        self.serializer_config_map
            .get::<T>()
            .and_then(|c| Some(c.clone()))
            .unwrap_or(Default::default())
    }

    pub fn try_new_serializer<W: io::Write>(
        &self,
        syntax_: Syntax,
        write: W,
    ) -> Result<DynSynQuadSerializer<W>, UnKnownSyntaxError> {
        match syntax_ {
            syntax::N_QUADS => Ok(DynSynQuadSerializer::new(InnerQuadSerializer::NQuads(
                NqSerializer::new_with_config(write, self.get_config::<NqConfig>()),
            ))),
            syntax::TRIG => Ok(DynSynQuadSerializer::new(InnerQuadSerializer::Trig(
                TrigSerializer::new_with_config(write, self.get_config::<TrigConfig>()),
            ))),
            _ => Err(UnKnownSyntaxError(syntax_)),
        }
    }

    pub fn try_new_stringifier(
        &self,
        syntax_: Syntax,
    ) -> Result<DynSynQuadSerializer<Vec<u8>>, UnKnownSyntaxError> {
        self.try_new_serializer(syntax_, Vec::new())
    }
}

// ---------------------------------------------------------------------------------
//                                      tests
// ---------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use once_cell::sync::Lazy;
    use sophia_api::{
        dataset::{isomorphic_datasets, Dataset},
        parser::QuadParser,
        quad::stream::QuadSource,
        serializer::{QuadSerializer, Stringifier},
    };
    use sophia_inmem::dataset::FastDataset;
    use sophia_term::BoxTerm;
    use sophia_turtle::serializer::{nq::NqConfig, trig::TrigConfig};
    use test_case::test_case;
    use type_map::concurrent::TypeMap;

    use crate::{
        parser::quads::DynSynQuadParserFactory,
        serializer::test_data::{TESTS_NQUADS, TESTS_TRIG},
        syntax::{self, Syntax},
        tests::TRACING,
    };

    use super::DynSynQuadSerializerFactory;

    static SERIALIZER_FACTORY: Lazy<DynSynQuadSerializerFactory> =
        Lazy::new(|| DynSynQuadSerializerFactory::new(TypeMap::new()));
    static SERIALIZER_FACTORY_WITH_PRETTY_CONFIG: Lazy<DynSynQuadSerializerFactory> =
        Lazy::new(|| {
            let mut config_map = TypeMap::new();
            config_map.insert::<TrigConfig>(TrigConfig::new().with_pretty(true));
            config_map.insert::<NqConfig>(NqConfig::default());

            DynSynQuadSerializerFactory {
                serializer_config_map: config_map,
            }
        });
    /// As DynSyn parsers can be non-cyclically tested, we can use them here.
    static QUAD_PARSER_FACTORY: Lazy<DynSynQuadParserFactory> =
        Lazy::new(|| DynSynQuadParserFactory::new());

    #[test_case(syntax::JSON_LD)]
    #[test_case(syntax::HTML_RDFA)]
    #[test_case(syntax::N_TRIPLES)]
    #[test_case(syntax::N3)]
    #[test_case(syntax::OWL2_XML)]
    #[test_case(syntax::TRIG)]
    #[test_case(syntax::XHTML_RDFA)]
    pub fn creating_parser_for_un_supported_syntax_will_error(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_err!(SERIALIZER_FACTORY.try_new_stringifier(syntax_));
    }

    #[test_case(syntax::N_QUADS)]
    #[test_case(syntax::TRIG)]
    pub fn creating_parser_for_supported_syntax_will_succeed(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_ok!(SERIALIZER_FACTORY.try_new_stringifier(syntax_));
    }

    #[test_case(syntax::TRIG, TESTS_TRIG[0], false)]
    #[test_case(syntax::TRIG, TESTS_TRIG[1], false)]
    #[test_case(syntax::TRIG, TESTS_TRIG[2], false)]
    #[test_case(syntax::TRIG, TESTS_TRIG[3], false)]
    #[test_case(syntax::TRIG, TESTS_TRIG[4], false)]
    #[test_case(syntax::TRIG, TESTS_TRIG[5], false)]
    #[test_case(syntax::TRIG, TESTS_TRIG[0], true)]
    #[test_case(syntax::TRIG, TESTS_TRIG[1], true)]
    #[test_case(syntax::TRIG, TESTS_TRIG[2], true)]
    #[test_case(syntax::TRIG, TESTS_TRIG[3], true)]
    #[test_case(syntax::TRIG, TESTS_TRIG[4], true)]
    #[test_case(syntax::TRIG, TESTS_TRIG[5], true)]
    #[test_case(syntax::N_QUADS, TESTS_NQUADS[0], false)]
    #[test_case(syntax::N_QUADS, TESTS_NQUADS[0], true)]
    pub fn correctly_roundtrips_for_syntax(syntax_: Syntax, rdf_doc: &str, pretty: bool) {
        let parser = QUAD_PARSER_FACTORY
            .try_new_parser(syntax_, None, None as Option<BoxTerm>)
            .unwrap();
        let d1: FastDataset = parser.parse_str(rdf_doc).collect_quads().unwrap();

        let factory = if pretty {
            &SERIALIZER_FACTORY_WITH_PRETTY_CONFIG
        } else {
            &SERIALIZER_FACTORY
        };

        let out = factory
            .try_new_stringifier(syntax_)
            .unwrap()
            .serialize_quads(d1.quads())
            .unwrap()
            .to_string();
        let d2: FastDataset = parser.parse_str(&out).collect_quads().unwrap();
        assert!(isomorphic_datasets(&d1, &d2).unwrap());
    }
}
