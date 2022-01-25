use std::io::BufRead;

use sophia_api::{
    parser::{QuadParser, TripleParser},
    term::{CopyTerm, TTerm},
};

use crate::syntax::{RdfSyntax, UnKnownSyntaxError};

use self::source::DynSynTripleSource;

use super::_inner::InnerParser;

pub mod source;

/// This parser implements [`sophia_api::parser::TripleParser`] trait, and can be instantiated at runtime against any of supported syntaxes using [`DynSynTripleParserFactory] factory.. It is generic over type of terms in triples it produces.
///
/// It can currently parse triples from documents in any of concrete_syntaxes: [`turtle`](crate::syntax::TURTLE), [`n-triples`](crate::syntax::N_TRIPLES), [rdf-xml](crate::syntax::RDF_XML), [`n-quads`](crate::syntax::N_QUADS), [`trig`](crate::syntax::TRIG). For docs in any of these syntaxes, this parser will stream quads through [`DynSynTripleSource`] instance.
///
/// For syntaxes that encodes quads instead of triples, like [`trig`](crate::syntax::TRIG), [`n-quads`](crate::syntax::N_QUADS), etc.. This parser can be configured with preferred graph_name term, to stream adapted triples from quads with specified graph_name. In that case, remaining underlying quads with different graph_name term will be ignored
///
/// Example:
///
/// ```
/// use rdf_dynsyn::{parser::triples::*, syntax};
///
/// use sophia_api::{graph::Graph, triple::stream::TripleSource, parser::TripleParser};
/// use sophia_inmem::graph::FastGraph;
/// use sophia_term::{matcher::ANY, BoxTerm, StaticTerm};
///
/// # pub fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// let parser_factory = DynSynTripleParserFactory::new();
///
/// let turtle_doc = r#"
///     @prefix : <http://example.org/ns/> .
///     <#me> :knows [ a :Person ; :name "Alice" ].
/// "#;
/// let doc_base_iri = "http://localhost/ex";
///
/// // A `DynSynQuadParser<BoxTerm>` instance, configured for trig syntax.
/// let parser = parser_factory.try_new_parser::<BoxTerm>(
///     syntax::TURTLE,
///     Some(doc_base_iri.into()),
///     None,
/// )?;
/// let mut graph = FastGraph::new();
/// let c = parser.parse_str(turtle_doc).add_to_graph(&mut graph)?;
///
/// assert_eq!(c, 3);
/// assert!(graph
///     .triples_matching(
///         &StaticTerm::new_iri("http://localhost/ex#me")?,
///         &StaticTerm::new_iri("http://example.org/ns/knows")?,
///         &ANY,
///     )
///     .next()
///     .is_some());
///
/// #     Ok(())
/// # }
/// # fn main() {try_main().unwrap();}
///```
///

#[derive(Debug)]
pub struct DynSynTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    inner_parser: InnerParser,
    quad_source_adapted_graph_iri: Option<T>,
}

impl<T> DynSynTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    pub fn try_new(
        syntax_: RdfSyntax,
        base_iri: Option<String>,
        quad_source_adapted_graph_iri: Option<T>,
    ) -> Result<Self, UnKnownSyntaxError> {
        let inner_parser = InnerParser::try_new(syntax_, base_iri)?;
        Ok(Self {
            inner_parser,
            quad_source_adapted_graph_iri,
        })
    }
}

impl<T, R> TripleParser<R> for DynSynTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
    R: BufRead,
{
    type Source = DynSynTripleSource<T, R>;

    fn parse(&self, data: R) -> Self::Source {
        let tsg_iri = self.quad_source_adapted_graph_iri.clone();
        // TODO may be abstract over literal repetition
        match &self.inner_parser {
            InnerParser::NQuads(p) => DynSynTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::TriG(p) => DynSynTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::NTriples(p) => DynSynTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::Turtle(p) => DynSynTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::RdfXml(p) => DynSynTripleSource::new_for(p.parse(data).into(), tsg_iri),
        }
    }
}

/// A factory to instantiate [`DynSynTripleParser`].
pub struct DynSynTripleParserFactory {}

impl DynSynTripleParserFactory {
    pub fn new() -> Self {
        Self {}
    }

    /// Try to create new [`DynSynTripleParser`] instance, for given `syntax_`, `base_iri`, and  `quad_source_adapted_graph_iri`.
    ///
    /// # Errors
    /// returns [`UnKnownSyntaxError`](crate::syntax::UnKnownSyntaxError) if requested syntax is not known/supported.
    pub fn try_new_parser<T>(
        &self,
        syntax_: RdfSyntax,
        base_iri: Option<String>,
        quad_source_adapted_graph_iri: Option<T>,
    ) -> Result<DynSynTripleParser<T>, UnKnownSyntaxError>
    where
        T: TTerm + CopyTerm + Clone,
    {
        DynSynTripleParser::try_new(syntax_, base_iri, quad_source_adapted_graph_iri)
    }
}

/// ---------------------------------------------------------------------------------
///                                      tests
/// ---------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use once_cell::sync::Lazy;
    use sophia_api::{
        dataset::Dataset,
        graph::isomorphic_graphs,
        parser::{IntoParsable, QuadParser, TripleParser},
        quad::stream::QuadSource,
        term::{CopyTerm, TTerm},
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

    use super::DynSynTripleParserFactory;
    use crate::parser::test_data::*;

    static DYNSYN_TRIPLE_PARSER_FACTORY: Lazy<DynSynTripleParserFactory> =
        Lazy::new(|| DynSynTripleParserFactory::new());

    #[test_case(syntax::JSON_LD)]
    #[test_case(syntax::HTML_RDFA)]
    #[test_case(syntax::N3)]
    #[test_case(syntax::OWL2_XML)]
    #[test_case(syntax::XHTML_RDFA)]
    pub fn creating_parser_for_un_supported_syntax_will_error(syntax_: RdfSyntax) {
        Lazy::force(&TRACING);
        assert_err!(&DYNSYN_TRIPLE_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    #[test_case(syntax::N_QUADS)]
    #[test_case(syntax::N_TRIPLES)]
    #[test_case(syntax::RDF_XML)]
    #[test_case(syntax::TRIG)]
    #[test_case(syntax::TURTLE)]
    pub fn creating_parser_for_supported_syntax_will_succeed(syntax_: RdfSyntax) {
        Lazy::force(&TRACING);
        assert_ok!(&DYNSYN_TRIPLE_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    fn check_graph_parse_isomorphism<'b, B, P1, P2>(p1: &P1, p2: &P2, qs: &'b str)
    where
        P1: TripleParser<B>,
        P2: TripleParser<B>,
        &'b str: IntoParsable<Target = B>,
    {
        let mut g1 = FastGraph::new();
        p1.parse_str(qs).add_to_graph(&mut g1).unwrap();

        let mut g2 = FastGraph::new();
        p2.parse_str(qs).add_to_graph(&mut g2).unwrap();

        assert!(isomorphic_graphs(&g1, &g2).unwrap());
    }

    fn check_dataset_parse_isomorphism<'b, B, P1, P2, T>(
        p1: &P1,
        p2: &P2,
        qs: &'b str,
        quad_source_virtual_graph_iri: Option<&T>,
    ) where
        P1: QuadParser<B>,
        P2: TripleParser<B>,
        &'b str: IntoParsable<Target = B>,
        T: TTerm + CopyTerm + Clone,
    {
        let mut d = FastDataset::new();
        p1.parse_str(qs).add_to_dataset(&mut d).unwrap();
        let g1 = d.graph(quad_source_virtual_graph_iri);

        let mut g2 = FastGraph::new();
        p2.parse_str(qs).add_to_graph(&mut g2).unwrap();

        assert!(isomorphic_graphs(&g1, &g2).unwrap());
    }

    #[test]
    pub fn correctly_parses_turtle() {
        Lazy::force(&TRACING);
        check_graph_parse_isomorphism(
            &TurtleParser {
                base: Some(BASE_IRI1.into()),
            },
            &DYNSYN_TRIPLE_PARSER_FACTORY
                .try_new_parser(
                    syntax::TURTLE,
                    Some(BASE_IRI1.into()),
                    None as Option<BoxTerm>,
                )
                .unwrap(),
            GRAPH_STR_TURTLE,
        );
    }

    #[test]
    pub fn correctly_parses_ntriples() {
        Lazy::force(&TRACING);
        check_graph_parse_isomorphism(
            &NTriplesParser {},
            &DYNSYN_TRIPLE_PARSER_FACTORY
                .try_new_parser(
                    syntax::N_TRIPLES,
                    Some(BASE_IRI1.into()),
                    None as Option<BoxTerm>,
                )
                .unwrap(),
            GRAPH_STR_NTRIPLES,
        );
    }

    #[test]
    pub fn correctly_parses_rdf_xml() {
        Lazy::force(&TRACING);
        check_graph_parse_isomorphism(
            &RdfXmlParser {
                base: Some(BASE_IRI1.into()),
            },
            &DYNSYN_TRIPLE_PARSER_FACTORY
                .try_new_parser(
                    syntax::RDF_XML,
                    Some(BASE_IRI1.into()),
                    None as Option<BoxTerm>,
                )
                .unwrap(),
            GRAPH_STR_RDF_XML,
        );
    }

    #[test_case(Some(G1_IRI))]
    #[test_case(Some(G2_IRI))]
    #[test_case(None)]
    pub fn correctly_parses_nquads(quad_source_virtual_graph_iri: Option<&str>) {
        Lazy::force(&TRACING);
        let quad_source_virtual_graph_iri = quad_source_virtual_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_dataset_parse_isomorphism(
            &NQuadsParser {},
            &DYNSYN_TRIPLE_PARSER_FACTORY
                .try_new_parser(
                    syntax::N_QUADS,
                    Some(BASE_IRI1.into()),
                    quad_source_virtual_graph_iri.clone(),
                )
                .unwrap(),
            DATASET_STR_NQUADS,
            quad_source_virtual_graph_iri.as_ref(),
        );
    }

    #[test_case(Some(G1_IRI))]
    #[test_case(Some(G2_IRI))]
    #[test_case(None)]
    pub fn correctly_parses_trig(quad_source_virtual_graph_iri: Option<&str>) {
        Lazy::force(&TRACING);
        let quad_source_virtual_graph_iri = quad_source_virtual_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_dataset_parse_isomorphism(
            &TriGParser {
                base: Some(BASE_IRI1.into()),
            },
            &DYNSYN_TRIPLE_PARSER_FACTORY
                .try_new_parser(
                    syntax::TRIG,
                    Some(BASE_IRI1.into()),
                    quad_source_virtual_graph_iri.clone(),
                )
                .unwrap(),
            DATASET_STR_TRIG,
            quad_source_virtual_graph_iri.as_ref(),
        );
    }
}
