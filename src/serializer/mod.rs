mod _inner;
pub mod quads;
pub mod triples;

#[cfg(test)]
mod test_data {
    //! These test data snippets are copied from sophia tests
    //!

    pub static TESTS_NQUADS: &[&str] = &[
        r#"<http://champin.net/#pa> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person>.
<http://champin.net/#pa> <http://schema.org/name> "Pierre-Antoine" <http://champin.net/>.
"#,
    ];

    pub static TESTS_NTRIPLES: &[&str] = &[
        r#"<http://champin.net/#pa> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person>.
<http://champin.net/#pa> <http://schema.org/name> "Pierre-Antoine".
"#,
    ];

    pub static TESTS_RDF_XML: &[&str] = &[r#"<?xml version="1.0" encoding="utf-8"?>
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
    "#];

    pub static TESTS_TRIG: &[&str] = &[
        "#empty trig",
        r#"# simple quads
            PREFIX : <http://example.org/ns/>
            :alice a :Person; :name "Alice"; :age 42.
            GRAPH :g {
                :bob a :Person, :Man; :nick "bob"@fr, "bobby"@en; :admin true.
            }
        "#,
        r#"# lists
            GRAPH <tag:g> { <tag:alice> <tag:likes> ( 1 2 ( 3 4 ) 5 6 ), ("a" "b"). }
        "#,
        r#"# subject lists
            GRAPH <tag:g> { (1 2 3) a <tag:List>. }
        "#,
        r#"# blank node graph name
            PREFIX : <http://example.org/ns/>
            #:lois :belives _:b.
            #GRAPH _:b1 { :clark a :Human }
        "#,
        r#"# list split over different graphs
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            _:a rdf:first 42; rdf:rest _:b.
            GRAPH [] {
                _:b rdf:first 43; rdf:rest ().
            }
        "#,
    ];

    pub static TESTS_TURTLE: &[&str] = &[
        "#empty ttl",
        r#"# simple triple
            PREFIX : <http://example.org/ns/>
            :alice a :Person; :name "Alice"; :age 42.
            :bob a :Person, :Man; :nick "bob"@fr, "bobby"@en; :admin true.
        "#,
        r#"# lists
            <tag:alice> <tag:likes> ( 1 2 ( 3 4 ) 5 6 ), ("a" "b").
        "#,
        r#"# subject lists
            (1 2 3) a <tag:List>.
        "#,
        r#"# malformed list
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            _:a rdf:first 42, 43; rdf:rest (44 45).
            _:b rdf:first 42; rdf:rest (43), (44).
        "#,
        r#"# bnode cycles
        PREFIX : <http://example.org/ns/>
        _:a :n "a"; :p [ :q [ :r _:a ]].
        _:b :n "b"; :s [ :s _:b ].
        "#,
    ];
}
