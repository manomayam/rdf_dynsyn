use std::io::BufRead;

use sophia_api::{
    parser::{QuadParser, TripleParser},
    term::{CopyTerm, TTerm},
};

use crate::syntax::Syntax;

use self::source::SomeHowTripleSource;

use super::{_inner::InnerParser, errors::UnKnownSyntaxError};

pub mod source;

#[derive(Debug)]
pub struct SomeHowTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    inner_parser: InnerParser,
    quad_source_virtual_graph_iri: Option<T>,
}

impl<T> SomeHowTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
{
    pub fn try_new(
        syntax_: Syntax,
        base_iri: Option<String>,
        quad_source_virtual_graph_iri: Option<T>,
    ) -> Result<Self, UnKnownSyntaxError> {
        let inner_parser = InnerParser::try_new(syntax_, base_iri)?;
        Ok(Self {
            inner_parser,
            quad_source_virtual_graph_iri,
        })
    }
}

impl<T, R> TripleParser<R> for SomeHowTripleParser<T>
where
    T: TTerm + CopyTerm + Clone,
    R: BufRead,
{
    type Source = SomeHowTripleSource<T, R>;

    fn parse(&self, data: R) -> Self::Source {
        let tsg_iri = self.quad_source_virtual_graph_iri.clone();
        // TODO may be abstract over literal repetition
        match &self.inner_parser {
            InnerParser::NQuads(p) => SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::TriG(p) => SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::NTriples(p) => SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::Turtle(p) => SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri),
            InnerParser::RdfXml(p) => SomeHowTripleSource::new_for(p.parse(data).into(), tsg_iri),
        }
    }
}

pub struct SomeHowTripleParserFactory {}

impl SomeHowTripleParserFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn try_new_parser<T>(
        &self,
        syntax_: Syntax,
        base_iri: Option<String>,
        quad_source_virtual_graph_iri: Option<T>,
    ) -> Result<SomeHowTripleParser<T>, UnKnownSyntaxError>
    where
        T: TTerm + CopyTerm + Clone,
    {
        SomeHowTripleParser::try_new(syntax_, base_iri, quad_source_virtual_graph_iri)
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
        quad::stream::QuadSource,
        term::{CopyTerm, TTerm},
        triple::{stream::TripleSource, Triple},
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

    use super::SomeHowTripleParserFactory;
    use crate::parser::test_data::*;

    static SOMEHOW_TRIPLE_PARSER_FACTORY: Lazy<SomeHowTripleParserFactory> =
        Lazy::new(|| SomeHowTripleParserFactory::new());

    #[test_case(syntax::JSON_LD)]
    #[test_case(syntax::HTML_RDFA)]
    #[test_case(syntax::N3)]
    #[test_case(syntax::OWL2_XML)]
    #[test_case(syntax::XHTML_RDFA)]
    pub fn creating_parser_for_un_supported_syntax_will_error(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_err!(&SOMEHOW_TRIPLE_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    #[test_case(syntax::N_QUADS)]
    #[test_case(syntax::N_TRIPLES)]
    #[test_case(syntax::RDF_XML)]
    #[test_case(syntax::TRIG)]
    #[test_case(syntax::TURTLE)]
    pub fn creating_parser_for_supported_syntax_will_succeed(syntax_: Syntax) {
        Lazy::force(&TRACING);
        assert_ok!(&SOMEHOW_TRIPLE_PARSER_FACTORY.try_new_parser::<BoxTerm>(syntax_, None, None));
    }

    fn check_dataset_parse_entailment<'b, B, P1, P2, T>(
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

        for t in g2.triples() {
            let t = t.unwrap();
            assert!(g1.contains(t.s(), t.p(), t.o()).unwrap());
        }
        for t in g1.triples() {
            let t = t.unwrap();
            assert!(g2.contains(t.s(), t.p(), t.o()).unwrap());
        }
    }

    fn check_graph_parse_entailment<'b, B, P1, P2>(p1: &P1, p2: &P2, qs: &'b str)
    where
        P1: TripleParser<B>,
        P2: TripleParser<B>,
        &'b str: IntoParsable<Target = B>,
    {
        let mut g1 = FastGraph::new();
        let c1 = p1.parse_str(qs).add_to_graph(&mut g1).unwrap();

        let mut g2 = FastGraph::new();
        let c2 = p2.parse_str(qs).add_to_graph(&mut g2).unwrap();

        assert_eq!(c1, c2);
        for t in g2.triples() {
            let t = t.unwrap();
            assert!(g1.contains(t.s(), t.p(), t.o()).unwrap());
        }
    }

    #[test_case(Some(G1_IRI))]
    #[test_case(Some(G2_IRI))]
    #[test_case(None)]
    pub fn correctly_parses_nquads(quad_source_virtual_graph_iri: Option<&str>) {
        let quad_source_virtual_graph_iri = quad_source_virtual_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_dataset_parse_entailment(
            &NQuadsParser {},
            &SOMEHOW_TRIPLE_PARSER_FACTORY
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
        let quad_source_virtual_graph_iri = quad_source_virtual_graph_iri
            .and_then(|v| Some(BoxTerm::Iri(Iri::new(Box::from(v)).unwrap())));
        check_dataset_parse_entailment(
            &TriGParser {
                base: Some(BASE_IRI1.into()),
            },
            &SOMEHOW_TRIPLE_PARSER_FACTORY
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

    #[test]
    pub fn correctly_parses_turtle() {
        check_graph_parse_entailment(
            &TurtleParser {
                base: Some(BASE_IRI1.into()),
            },
            &SOMEHOW_TRIPLE_PARSER_FACTORY
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
        check_graph_parse_entailment(
            &NTriplesParser {},
            &SOMEHOW_TRIPLE_PARSER_FACTORY
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
        check_graph_parse_entailment(
            &RdfXmlParser {
                base: Some(BASE_IRI1.into()),
            },
            &SOMEHOW_TRIPLE_PARSER_FACTORY
                .try_new_parser(
                    syntax::RDF_XML,
                    Some(BASE_IRI1.into()),
                    None as Option<BoxTerm>,
                )
                .unwrap(),
            GRAPH_STR_RDF_XML,
        );
    }
}
