use std::io::BufRead;

use sophia_api::{
    parser::{QuadParser, TripleParser},
    term::{CopyTerm, TTerm},
};

use crate::syntax::{RdfSyntax, UnKnownSyntaxError};

use self::source::DynSynQuadSource;

use super::_inner::InnerParser;

pub mod source;

/// This parser implements [`sophia_api::parser::QuadParser`] trait, and can be instantiated at runtime against any of supported syntaxes using [`DynSynQuadParserFactory`] factory. It is generic over type of terms in quads it produces.
///
/// It can currently parse quads from documents in any of concrete_syntaxes: [`n-quads`](crate::syntax::N_QUADS), [`trig`](crate::syntax::TRIG), [`turtle`](crate::syntax::TURTLE), [`n-triples`](crate::syntax::N_TRIPLES), [rdf-xml](crate::syntax::RDF_XML). For docs in any of these syntaxes, this parser will stream quads through [`DynSynQuadSource`] instance.
///
/// For syntaxes that doesn't support quads, like [`turtle`](crate::syntax::TURTLE), [`n-triples`](crate::syntax::N_TRIPLES), [rdf-xml](crate::syntax::RDF_XML), etc.. This parser can be configured with preferred graph_name term for quads that are adapted from underlying triples.
///
/// Example:
///
/// ```
/// use rdf_dynsyn::{parser::quads::*, syntax};
///
/// use sophia_api::{dataset::Dataset, quad::stream::QuadSource, parser::QuadParser};
/// use sophia_inmem::dataset::FastDataset;
/// use sophia_term::{matcher::ANY, BoxTerm, StaticTerm};
///
/// # pub fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let parser_factory = DynSynQuadParserFactory::new();
///
/// let trig_doc = r#"
///     @prefix : <http://example.org/ns/> .
///     <#g1> {
///         <#me> :knows _:alice.
///     }
///     <#g2> {
///         _:alice a :Person ; :name "Alice".
///     }
/// "#;
/// let doc_base_iri = "http://localhost/ex";
///
/// // A `DynSynQuadParser<BoxTerm>` instance, configured for trig syntax.
/// let parser = parser_factory.try_new_parser::<BoxTerm>(
///     syntax::TRIG,
///     Some(doc_base_iri.into()),
///     None,
/// )?;
/// let mut dataset = FastDataset::new();
/// let c = parser.parse_str(trig_doc).add_to_dataset(&mut dataset)?;
///
/// assert_eq!(c, 3);
/// assert!(dataset
///     .quads_matching(
///         &StaticTerm::new_iri("http://localhost/ex#me")?,
///         &StaticTerm::new_iri("http://example.org/ns/knows")?,
///         &ANY,
///         &Some(&StaticTerm::new_iri("http://localhost/ex#g1")?),
///     )
///     .next()
///     .is_some());
/// #     Ok(())
/// # }
/// # fn main() {try_main().unwrap();}
///```
///

#[derive(Debug)]
pub struct DynSynQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    inner_parser: InnerParser,
    triple_source_adapted_graph_iri: Option<T>,
}

impl<T> DynSynQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    pub(crate) fn try_new(
        syntax_: RdfSyntax,
        base_iri: Option<String>,
        triple_source_adapted_graph_iri: Option<T>,
    ) -> Result<Self, UnKnownSyntaxError> {
        let inner_parser = InnerParser::try_new(syntax_, base_iri)?;
        Ok(Self {
            inner_parser,
            triple_source_adapted_graph_iri,
        })
    }
}

impl<T, R> QuadParser<R> for DynSynQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
    R: BufRead,
{
    type Source = DynSynQuadSource<T, R>;

    fn parse(&self, data: R) -> Self::Source {
        let tsg_iri = self.triple_source_adapted_graph_iri.clone();
        // TODO may have to abstract over literal repetition
        match &self.inner_parser {
            InnerParser::NQuads(p) => DynSynQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::TriG(p) => DynSynQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::NTriples(p) => DynSynQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::Turtle(p) => DynSynQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::RdfXml(p) => DynSynQuadSource::new_for(p.parse(data).into(), tsg_iri),
        }
    }
}

/// A factory to instantiate [`DynSynQuadParser`].
pub struct DynSynQuadParserFactory {}

impl DynSynQuadParserFactory {
    pub fn new() -> Self {
        Self {}
    }

    //// Try to create new [`DynSynQuadParser`] instance, for given `syntax_`, `base_iri`, and  `triple_source_adapted_graph_iri`.
    ////
    //// # Errors
    //// returns [`UnKnownSyntaxError`] if requested syntax is not known/supported.
    pub fn try_new_parser<T>(
        &self,
        syntax_: RdfSyntax,
        base_iri: Option<String>,
        triple_source_adapted_graph_iri: Option<T>,
    ) -> Result<DynSynQuadParser<T>, UnKnownSyntaxError>
    where
        T: TTerm + CopyTerm + Clone,
    {
        DynSynQuadParser::try_new(syntax_, base_iri, triple_source_adapted_graph_iri)
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
        graph::Graph,
        parser::{IntoParsable, QuadParser, TripleParser},
        quad::{stream::QuadSource, Quad},
        term::{term_eq, CopyTerm, TTerm},
        triple::stream::TripleSource,
    };
    use sophia_inmem::{dataset::FastDataset, graph::FastGraph};
    use sophia_term::{iri::Iri, BoxTerm};
    use sophia_turtle::parser::{
        nq::NQuadsParser, nt::NTriplesParser, trig::TriGParser, turtle::TurtleParser,
    };
    use sophia_xml::parser::RdfXmlParser;
    use test_case::test_case;

    use crate::{
        syntax::{self, RdfSyntax},
        tests::TRACING,
    };

    use super::DynSynQuadParserFactory;
    use crate::parser::test_data::*;

    static DYNSYN_QUAD_PARSER_FACTORY: Lazy<DynSynQuadParserFactory> =
        Lazy::new(|| DynSynQuadParserFactory::new());

    #[test_case(syntax::JSON_LD)]
    #[test_case(syntax::HTML_RDFA)]
    #[test_case(syntax::N3)]
    #[test_case(syntax::OWL2_XML)]
    #[test_case(syntax::XHTML_RDFA)]
    pub fn creating_parser_for_un_supported_syntax_will_error(syntax_: RdfSyntax) {
        Lazy::force(&TRACING);
        assert_err!(&DYNSYN_QUAD_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    #[test_case(syntax::N_QUADS)]
    #[test_case(syntax::N_TRIPLES)]
    #[test_case(syntax::RDF_XML)]
    #[test_case(syntax::TRIG)]
    #[test_case(syntax::TURTLE)]
    pub fn creating_parser_for_supported_syntax_will_succeed(syntax_: RdfSyntax) {
        Lazy::force(&TRACING);
        assert_ok!(&DYNSYN_QUAD_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    fn check_dataset_parse_isomorphism<'b, B, P1, P2>(p1: &P1, p2: &P2, qs: &'b str)
    where
        P1: QuadParser<B>,
        P2: QuadParser<B>,
        &'b str: IntoParsable<Target = B>,
    {
        let mut d1 = FastDataset::new();
        p1.parse_str(qs).add_to_dataset(&mut d1).unwrap();

        let mut d2 = FastDataset::new();
        p2.parse_str(qs).add_to_dataset(&mut d2).unwrap();

        assert!(isomorphic_datasets(&d1, &d2).unwrap());
    }

    fn check_graph_parse_isomorphism<'b, B, P1, P2, T>(
        p1: &P1,
        p2: &P2,
        qs: &'b str,
        triple_source_graph_iri: Option<&T>,
    ) where
        P1: TripleParser<B>,
        P2: QuadParser<B>,
        &'b str: IntoParsable<Target = B>,
        T: TTerm + CopyTerm + Clone,
    {
        let mut g = FastGraph::new();
        let c1 = p1.parse_str(qs).add_to_graph(&mut g).unwrap();

        let mut d = FastDataset::new();
        let c2 = p2.parse_str(qs).add_to_dataset(&mut d).unwrap();

        assert_eq!(c1, c2);
        for q in d.quads() {
            let q = q.unwrap();
            assert!(g.contains(q.s(), q.p(), q.o()).unwrap());
            assert!(match (q.g(), triple_source_graph_iri) {
                (None, None) => true,
                (Some(g_iri1), Some(g_iri2)) => term_eq(g_iri1, g_iri2),
                _ => false,
            });
        }
    }

    #[test]
    pub fn correctly_parses_nquads() {
        Lazy::force(&TRACING);
        check_dataset_parse_isomorphism(
            &NQuadsParser {},
            &DYNSYN_QUAD_PARSER_FACTORY
                .try_new_parser::<BoxTerm>(syntax::N_QUADS, Some(BASE_IRI1.into()), None)
                .unwrap(),
            DATASET_STR_NQUADS,
        );
    }

    #[test]
    pub fn correctly_parses_trig() {
        Lazy::force(&TRACING);
        check_dataset_parse_isomorphism(
            &TriGParser {
                base: Some(BASE_IRI1.into()),
            },
            &DYNSYN_QUAD_PARSER_FACTORY
                .try_new_parser::<BoxTerm>(syntax::TRIG, Some(BASE_IRI1.into()), None)
                .unwrap(),
            DATASET_STR_TRIG,
        );
    }

    #[test_case(Some(G1_IRI))]
    #[test_case(Some(G2_IRI))]
    #[test_case(None)]
    pub fn correctly_parses_turtle(triple_source_graph_iri: Option<&str>) {
        Lazy::force(&TRACING);
        let triple_source_graph_iri = triple_source_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_graph_parse_isomorphism(
            &TurtleParser {
                base: Some(BASE_IRI1.into()),
            },
            &DYNSYN_QUAD_PARSER_FACTORY
                .try_new_parser(
                    syntax::TURTLE,
                    Some(BASE_IRI1.into()),
                    triple_source_graph_iri.clone(),
                )
                .unwrap(),
            GRAPH_STR_TURTLE,
            triple_source_graph_iri.as_ref(),
        );
    }

    #[test_case(Some(G1_IRI))]
    #[test_case(Some(G2_IRI))]
    #[test_case(None)]
    pub fn correctly_parses_ntriples(triple_source_graph_iri: Option<&str>) {
        Lazy::force(&TRACING);
        let triple_source_graph_iri = triple_source_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_graph_parse_isomorphism(
            &NTriplesParser {},
            &DYNSYN_QUAD_PARSER_FACTORY
                .try_new_parser::<BoxTerm>(
                    syntax::N_TRIPLES,
                    Some(BASE_IRI1.into()),
                    triple_source_graph_iri.clone(),
                )
                .unwrap(),
            GRAPH_STR_NTRIPLES,
            triple_source_graph_iri.as_ref(),
        );
    }

    #[test_case(Some(G1_IRI))]
    #[test_case(Some(G2_IRI))]
    #[test_case(None)]
    pub fn correctly_parses_rdf_xml(triple_source_graph_iri: Option<&str>) {
        Lazy::force(&TRACING);
        let triple_source_graph_iri = triple_source_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_graph_parse_isomorphism(
            &RdfXmlParser {
                base: Some(BASE_IRI1.into()),
            },
            &DYNSYN_QUAD_PARSER_FACTORY
                .try_new_parser::<BoxTerm>(
                    syntax::RDF_XML,
                    Some(BASE_IRI1.into()),
                    triple_source_graph_iri.clone(),
                )
                .unwrap(),
            GRAPH_STR_RDF_XML,
            triple_source_graph_iri.as_ref(),
        );
    }
}
