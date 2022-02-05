# rdf_dynsyn

This crate provides sophia-compatible and sophia-based rdf parsers/serializers, that can be instantiated against any of supported syntaxes dynamically at run time.

## Why?
Although sophia provides specialized parsers/serializers for each syntax, we have to know document syntax at code-time to practically use them. In many cases of web, we may know syntax of a doc only at runtime, like from content-type, file-extn, etc. As each specialized parser parses to corresponding stream types, etc.. it will be difficult to work with them in such dynamic cases. For Handling such cases this crate provides well-tested abstractions, that integrates into sophia eco-system.

## Getting Started

Following a short example how to get syntax from media-types/file-extensions, and instantiate parser for detected syntax, parse content,mutate it  and serialize back into desired syntax. Also see documentation for more.

```rust
use std::str::FromStr;

use mime::Mime;
use sophia_api::{
    graph::MutableGraph,
    ns::Namespace,
    parser::TripleParser,
    serializer::{Stringifier, TripleSerializer},
    triple::stream::TripleSource,
};
use sophia_inmem::graph::FastGraph;
use sophia_term::BoxTerm;

use rdf_dynsyn::{
    correspondence::Correspondent, parser::triples::DynSynTripleParserFactory,
    serializer::triples::DynSynTripleSerializerFactory, syntax::RdfSyntax,
};

    //  let's say following are input params, we get dynamically. media_type, content of source doc, and target media_type to convert into
    let src_doc_media_type = "text/turtle";
    let tgt_doc_media_type = "application/rdf+xml";
    let src_doc_content = r#"
        @prefix : <http://example.org/>.
        @prefix foaf: <http://xmlns.com/foaf/0.1/>.

        :alice foaf:name "Alice";
            foaf:mbox <mailto:alice@work.example> .
        :bob foaf:name "Bob".
    "#;

    // resolve syntaxes for media_types. Or one can use static constants exported by `syntax` module,
    let src_doc_syntax =
        Correspondent::<RdfSyntax>::try_from(&Mime::from_str(src_doc_media_type)?)?.value;
    let tgt_doc_syntax =
        Correspondent::<RdfSyntax>::try_from(&Mime::from_str(tgt_doc_media_type)?)?.value;

    // get parser for source syntax
    let parser_factory = DynSynTripleParserFactory::default();
    let parser = parser_factory.try_new_parser::<BoxTerm>(src_doc_syntax, None, None)?;

    // parse to a graph
    let mut graph: FastGraph = parser.parse_str(src_doc_content).collect_triples()?;

    // mutate graph
    let ex = Namespace::new("http://example.org/")?;
    let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;
    graph.insert(&ex.get("bob")?, &foaf.get("knows")?, &ex.get("alice")?)?;

    // get serializer for target syntax
    let serializer_factory = DynSynTripleSerializerFactory::new(None); // Here we can pass optional formatting options. see documentation.
    let mut serializer = serializer_factory.try_new_stringifier(tgt_doc_syntax)?;
    let serialized_doc = serializer.serialize_graph(&graph)?.as_str();
    println!("The resulting graph\n{}", serialized_doc);
```


License: MIT OR Apache-2.0
