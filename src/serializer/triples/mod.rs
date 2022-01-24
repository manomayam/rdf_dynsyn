use std::io;

use sophia_api::serializer::{Stringifier, TripleSerializer};
use sophia_turtle::serializer::{
    nt::{NtConfig, NtSerializer},
    turtle::{TurtleConfig, TurtleSerializer},
};
use sophia_xml::serializer::{RdfXmlConfig, RdfXmlSerializer};
use type_map::concurrent::TypeMap;

use crate::{
    parser::errors::UnKnownSyntaxError,
    syntax::{self, Syntax},
};

use super::_inner::InnerTripleSerializer;

#[derive(Debug)]
pub struct DynSynTripleSerializer<W: io::Write> {
    inner_serializer: InnerTripleSerializer<W>,
}

impl<W: io::Write> DynSynTripleSerializer<W> {
    pub(crate) fn new(inner_serializer: InnerTripleSerializer<W>) -> Self {
        Self { inner_serializer }
    }
}

impl Stringifier for DynSynTripleSerializer<Vec<u8>> {
    fn as_utf8(&self) -> &[u8] {
        match &self.inner_serializer {
            InnerTripleSerializer::NTriples(s) => s.as_utf8(),
            InnerTripleSerializer::Turtle(s) => s.as_utf8(),
            InnerTripleSerializer::RdfXml(s) => s.as_utf8(),
        }
    }
}

impl<W: io::Write> TripleSerializer for DynSynTripleSerializer<W> {
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

pub struct DynSynTripleSerializerFactory {
    serializer_config_map: TypeMap,
}

impl DynSynTripleSerializerFactory {
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
    ) -> Result<DynSynTripleSerializer<W>, UnKnownSyntaxError> {
        match syntax_ {
            syntax::N_TRIPLES => Ok(DynSynTripleSerializer::new(
                InnerTripleSerializer::NTriples(NtSerializer::new_with_config(
                    write,
                    self.get_config::<NtConfig>(),
                )),
            )),
            syntax::TURTLE => Ok(DynSynTripleSerializer::new(InnerTripleSerializer::Turtle(
                TurtleSerializer::new_with_config(write, self.get_config::<TurtleConfig>()),
            ))),
            syntax::RDF_XML => Ok(DynSynTripleSerializer::new(InnerTripleSerializer::RdfXml(
                RdfXmlSerializer::new_with_config(write, self.get_config::<RdfXmlConfig>()),
            ))),
            _ => Err(UnKnownSyntaxError(syntax_)),
        }
    }

    pub fn try_new_stringifier(
        &self,
        syntax_: Syntax,
    ) -> Result<DynSynTripleSerializer<Vec<u8>>, UnKnownSyntaxError> {
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
        graph::{isomorphic_graphs, Graph},
        parser::TripleParser,
        serializer::{Stringifier, TripleSerializer},
        triple::stream::TripleSource,
    };
    use sophia_inmem::graph::FastGraph;
    use sophia_term::BoxTerm;
    use sophia_turtle::serializer::{nt::NtConfig, turtle::TurtleConfig};
    use sophia_xml::serializer::RdfXmlConfig;
    use test_case::test_case;
    use type_map::concurrent::TypeMap;

    use crate::{
        parser::triples::DynSynTripleParserFactory,
        serializer::test_data::{TESTS_NTRIPLES, TESTS_RDF_XML, TESTS_TURTLE},
        syntax::{self, Syntax},
        tests::TRACING,
    };

    use super::DynSynTripleSerializerFactory;

    static SERIALIZER_FACTORY: Lazy<DynSynTripleSerializerFactory> =
        Lazy::new(|| DynSynTripleSerializerFactory::new(TypeMap::new()));

    static SERIALIZER_FACTORY_WITH_PRETTY_CONFIG: Lazy<DynSynTripleSerializerFactory> =
        Lazy::new(|| {
            let mut config_map = TypeMap::new();
            config_map.insert::<TurtleConfig>(TurtleConfig::new().with_pretty(true));
            config_map.insert::<NtConfig>(NtConfig::default());
            config_map.insert::<RdfXmlConfig>(RdfXmlConfig::default());

            DynSynTripleSerializerFactory {
                serializer_config_map: config_map,
            }
        });

    /// As DynSyn parsers can be non-cyclically tested, we can use them here.
    static TRIPLE_PARSER_FACTORY: Lazy<DynSynTripleParserFactory> =
        Lazy::new(|| DynSynTripleParserFactory::new());

    #[test_case(syntax::JSON_LD)]
    #[test_case(syntax::HTML_RDFA)]
    #[test_case(syntax::N_QUADS)]
    #[test_case(syntax::N3)]
    #[test_case(syntax::OWL2_XML)]
    #[test_case(syntax::TRIG)]
    #[test_case(syntax::XHTML_RDFA)]
    pub fn creating_parser_for_un_supported_syntax_will_error(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_err!(SERIALIZER_FACTORY.try_new_serializer(syntax_, Vec::new()));
    }

    #[test_case(syntax::N_TRIPLES)]
    #[test_case(syntax::RDF_XML)]
    #[test_case(syntax::TURTLE)]
    pub fn creating_parser_for_supported_syntax_will_succeed(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_ok!(SERIALIZER_FACTORY.try_new_stringifier(syntax_));
    }

    #[test_case(syntax::TURTLE, TESTS_TURTLE[0], false)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[1], false)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[2], false)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[3], false)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[4], false)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[5], false)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[0], true)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[1], true)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[2], true)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[3], true)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[4], true)]
    #[test_case(syntax::TURTLE, TESTS_TURTLE[5], true)]
    #[test_case(syntax::N_TRIPLES, TESTS_NTRIPLES[0], false)]
    #[test_case(syntax::N_TRIPLES, TESTS_NTRIPLES[0], true)]
    #[test_case(syntax::RDF_XML, TESTS_RDF_XML[0], false)]
    #[test_case(syntax::RDF_XML, TESTS_RDF_XML[0], true)]
    pub fn correctly_roundtrips_for_syntax(syntax_: Syntax, rdf_doc: &str, pretty: bool) {
        Lazy::force(&TRACING);
        let parser = TRIPLE_PARSER_FACTORY
            .try_new_parser(syntax_, None, None as Option<BoxTerm>)
            .unwrap();
        let g1: FastGraph = parser.parse_str(rdf_doc).collect_triples().unwrap();

        let factory = if pretty {
            &SERIALIZER_FACTORY_WITH_PRETTY_CONFIG
        } else {
            &SERIALIZER_FACTORY
        };

        let out = factory
            .try_new_stringifier(syntax_)
            .unwrap()
            .serialize_triples(g1.triples())
            .unwrap()
            .to_string();
        let g2: FastGraph = parser.parse_str(&out).collect_triples().unwrap();
        assert!(isomorphic_graphs(&g1, &g2).unwrap());
    }
}
