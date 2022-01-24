mod _inner;
pub mod errors;
pub mod quads;
pub mod triples;

#[cfg(test)]
pub mod test_data {
    // Is copied from sophia tests
    pub static DATASET_STR_NQUADS: &str = r#"
        <http://localhost/ex#me> <http://example.org/ns/knows> _:b1.
        _:b1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://example.org/ns/Person> <tag:g1>.
        _:b1 <http://example.org/ns/name> "Alice" <tag:g1>.
    "#;

    pub static DATASET_STR_TRIG: &str = r#"
        @prefix : <http://example.org/ns/> .
        <#g1> {
            <#me> :knows _:alice.
        }
        <#g2> {
            _:alice a :Person ; :name "Alice".
        }
    "#;

    pub static GRAPH_STR_TURTLE: &'static str = r#"
        @prefix : <http://example.org/ns/> .
        <#me> :knows [ a :Person ; :name "Alice" ].
    "#;

    pub static GRAPH_STR_NTRIPLES: &'static str = r#"
        <http://localhost/ex#me> <http://example.org/ns/knows> _:b1.
        _:b1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://example.org/ns/Person>.
        _:b1 <http://example.org/ns/name> "Alice".
    "#;

    pub static GRAPH_STR_RDF_XML: &'static str = r#"<?xml version="1.0" encoding="utf-8"?>
    <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
                xmlns="http://example.org/ns/">
        <rdf:Description rdf:about="http://localhost/ex#me">
        <knows>
            <Person>
            <name>Alice</name>
            </Person>
        </knows>
        </rdf:Description>
    </rdf:RDF>
    "#;

    pub static BASE_IRI1: &'static str = "http://localhost/ex";
    pub static G1_IRI: &'static str = "http://localhost/ex#g1";
    pub static G2_IRI: &'static str = "http://localhost/ex#g2";
}
