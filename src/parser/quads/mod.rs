use std::io::BufRead;

use sophia_api::{
    parser::{QuadParser, TripleParser},
    term::{CopyTerm, TTerm},
};

use crate::syntax::Syntax;

use self::source::SomeHowQuadSource;

use super::{_inner::InnerParser, errors::UnKnownSyntaxError};

pub mod source;

#[derive(Debug)]
pub struct SomeHowQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    inner_parser: InnerParser,
    triple_source_graph_iri: Option<T>,
}

impl<T> SomeHowQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    pub(crate) fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
        triple_source_graph_iri: Option<T>,
    ) -> Result<Self, UnKnownSyntaxError> {
        let inner_parser = InnerParser::try_new(syntax_, base_iri)?;
        Ok(Self {
            inner_parser,
            triple_source_graph_iri,
        })
    }
}

impl<T, R> QuadParser<R> for SomeHowQuadParser<T>
where
    T: TTerm + CopyTerm + Clone,
    R: BufRead,
{
    type Source = SomeHowQuadSource<T, R>;

    fn parse(&self, data: R) -> Self::Source {
        let tsg_iri = self.triple_source_graph_iri.clone();
        // TODO may have to abstract over literal repetition
        match &self.inner_parser {
            InnerParser::NQuads(p) => SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::TriG(p) => SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::NTriples(p) => SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::Turtle(p) => SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::RdfXml(p) => SomeHowQuadSource::new_for(p.parse(data).into(), tsg_iri),
        }
    }
}

pub struct SomeHowQuadParserFactory {}

impl SomeHowQuadParserFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn try_new_parser<T>(
        &self,
        syntax_: Syntax,
        base_iri: Option<String>,
        triple_source_graph_iri: Option<T>,
    ) -> Result<SomeHowQuadParser<T>, UnKnownSyntaxError>
    where
        T: TTerm + CopyTerm + Clone,
    {
        SomeHowQuadParser::try_new(syntax_, base_iri, triple_source_graph_iri)
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use once_cell::sync::Lazy;
    use sophia_api::{
        dataset::Dataset,
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
        syntax::{self, Syntax},
        tests::TRACING,
    };

    use super::SomeHowQuadParserFactory;
    use crate::parser::test_data::*;

    static SOMEHOW_QUAD_PARSER_FACTORY: Lazy<SomeHowQuadParserFactory> =
        Lazy::new(|| SomeHowQuadParserFactory::new());

    #[test_case(syntax::JSON_LD)]
    #[test_case(syntax::HTML_RDFA)]
    #[test_case(syntax::N3)]
    #[test_case(syntax::OWL2_XML)]
    #[test_case(syntax::XHTML_RDFA)]
    pub fn creating_parser_for_un_supported_syntax_will_error(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_err!(&SOMEHOW_QUAD_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    #[test_case(syntax::N_QUADS)]
    #[test_case(syntax::N_TRIPLES)]
    #[test_case(syntax::RDF_XML)]
    #[test_case(syntax::TRIG)]
    #[test_case(syntax::TURTLE)]
    pub fn creating_parser_for_supported_syntax_will_succeed(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_ok!(&SOMEHOW_QUAD_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    fn check_dataset_parse_entailment<'b, B, P1, P2>(p1: &P1, p2: &P2, qs: &'b str)
    where
        P1: QuadParser<B>,
        P2: QuadParser<B>,
        &'b str: IntoParsable<Target = B>,
    {
        let mut d1 = FastDataset::new();
        let c1 = p1.parse_str(qs).add_to_dataset(&mut d1).unwrap();

        let mut d2 = FastDataset::new();
        let c2 = p2.parse_str(qs).add_to_dataset(&mut d2).unwrap();

        assert_eq!(c1, c2);
        for q in d1.quads() {
            let q = q.unwrap();
            assert!(d2.contains(q.s(), q.p(), q.o(), q.g()).unwrap());
        }
    }

    fn check_graph_parse_entailment<'b, B, P1, P2, T>(
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
        check_dataset_parse_entailment(
            &NQuadsParser {},
            &SOMEHOW_QUAD_PARSER_FACTORY
                .try_new_parser(
                    syntax::N_QUADS,
                    Some(BASE_IRI1.into()),
                    None as Option<BoxTerm>,
                )
                .unwrap(),
            DATASET_STR_NQUADS,
        );
    }

    #[test]
    pub fn correctly_parses_trig() {
        check_dataset_parse_entailment(
            &TriGParser {
                base: Some(BASE_IRI1.into()),
            },
            &SOMEHOW_QUAD_PARSER_FACTORY
                .try_new_parser(
                    syntax::TRIG,
                    Some(BASE_IRI1.into()),
                    None as Option<BoxTerm>,
                )
                .unwrap(),
            DATASET_STR_TRIG,
        );
    }

    #[test_case(Some(G1_IRI))]
    #[test_case(Some(G2_IRI))]
    #[test_case(None)]
    pub fn correctly_parses_turtle(triple_source_graph_iri: Option<&str>) {
        let triple_source_graph_iri = triple_source_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_graph_parse_entailment(
            &TurtleParser {
                base: Some(BASE_IRI1.into()),
            },
            &SOMEHOW_QUAD_PARSER_FACTORY
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
        let triple_source_graph_iri = triple_source_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_graph_parse_entailment(
            &NTriplesParser {},
            &SOMEHOW_QUAD_PARSER_FACTORY
                .try_new_parser(
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
        let triple_source_graph_iri = triple_source_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_graph_parse_entailment(
            &RdfXmlParser {
                base: Some(BASE_IRI1.into()),
            },
            &SOMEHOW_QUAD_PARSER_FACTORY
                .try_new_parser(
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
