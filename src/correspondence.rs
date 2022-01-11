use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::{
    file_extension::{self as fextn, FileExtension},
    media_type,
    syntax::{self, Syntax},
};

#[derive(Debug, Clone)]
/// A struct that wraps a corresponding value for some other entity, and qualifies correspondence with exclusivity
pub struct Correspondent<T> {
    /// correspondent value
    pub value: T,
    /// wether correspondence is total
    pub is_total: bool,
}

macro_rules! set_correspondence {
    ($map:ident; $($k:expr, $v:expr, $t:expr;)*) => {
        $(
            $map.insert($k, Correspondent { value: $v, is_total: $t });
        )*
    };
}

/// A mapping from known rdf syntaxes to their canonical corresponding preferred file-extensions
pub static SYNTAX_TO_EXTENSION_CORRESPONDENCE: Lazy<HashMap<Syntax, Correspondent<FileExtension>>> =
    Lazy::new(|| {
        let mut map: HashMap<Syntax, Correspondent<FileExtension>> = HashMap::new();
        set_correspondence!(
            map;
            syntax::HTML_RDFA, fextn::HTML, true;

            syntax::JSON_LD, fextn::JSONLD, true;

            syntax::N3, fextn::N3, true;

            syntax::N_QUADS, fextn::NQ, true;

            syntax::N_TRIPLES, fextn::NT, true;

            syntax::OWL2_MANCHESTER, fextn::OMN, true;

            syntax::OWL2_XML, fextn::OWL, true;

            syntax::RDF_XML, fextn::RDF, true;

            syntax::TRIG, fextn::TRIG, true;

            syntax::TURTLE, fextn::TTL, true;

            syntax::XHTML_RDFA, fextn::XHTML, true;
        );
        map
    });

/// A mapping from known file-extensions for rdf documents to their canonical  corresponding syntaxes
pub static EXTENSION_TO_SYNTAX_CORRESPONDENCE: Lazy<HashMap<FileExtension, Correspondent<Syntax>>> =
    Lazy::new(|| {
        let mut map: HashMap<FileExtension, Correspondent<Syntax>> = HashMap::new();
        set_correspondence!(
            map;
            fextn::HTML, syntax::HTML_RDFA, false;

            fextn::JSONLD, syntax::JSON_LD, true;

            fextn::JSON, syntax::JSON_LD, false;

            fextn::N3, syntax::N3, true;

            fextn::NQ, syntax::N_QUADS, true;

            fextn::NQUADS, syntax::N_QUADS, true;

            fextn::NT, syntax::N_TRIPLES, true;

            fextn::NTRIPLES, syntax::N_TRIPLES, true;

            fextn::OMN, syntax::OWL2_MANCHESTER, true;

            fextn::OWL, syntax::OWL2_XML, true;

            fextn::OWX, syntax::OWL2_XML, true;

            fextn::RDF, syntax::RDF_XML, true;

            fextn::RDFXML, syntax::RDF_XML, true;

            fextn::TRIG, syntax::TRIG, true;

            fextn::TTL, syntax::TURTLE, true;

            fextn::TURTLE, syntax::TURTLE, true;

            fextn::XHTML, syntax::XHTML_RDFA, false;
        );
        map
    });

/// A mapping from known rdf syntaxes to their canonical  corresponding media-types
pub static SYNTAX_TO_MEDIA_TYPE_CORRESPONDENCE: Lazy<
    HashMap<Syntax, Correspondent<&'static mime::Mime>>,
> = Lazy::new(|| {
    let mut map: HashMap<Syntax, Correspondent<&'static mime::Mime>> = HashMap::new();
    set_correspondence!(
        map;
        syntax::HTML_RDFA, &media_type::TEXT_HTML, true;

        syntax::JSON_LD, &media_type::APPLICATION_JSON_LD, true;

        syntax::N3, &media_type::TEXT_N3, true;

        syntax::N_QUADS, &media_type::APPLICATION_N_QUADS, true;

        syntax::N_TRIPLES, &media_type::APPLICATION_N_TRIPLES, true;

        syntax::OWL2_MANCHESTER, &media_type::TEXT_OWL_MANCHESTER, true;

        syntax::OWL2_XML, &media_type::APPLICATION_OWL_XML, true;

        syntax::RDF_XML, &media_type::APPLICATION_RDF_XML, true;

        syntax::TRIG, &media_type::APPLICATION_TRIG, true;

        syntax::TURTLE, &media_type::TEXT_TURTLE, true;

        syntax::XHTML_RDFA, &media_type::APPLICATION_XHTML_XML, true;
    );
    map
});

/// A mapping from known media-types for rdf documents to their canonical  corresponding syntaxes
pub static MEDIA_TYPE_TO_SYNTAX_CORRESPONDENCE: Lazy<
    HashMap<&'static mime::Mime, Correspondent<Syntax>>,
> = Lazy::new(|| {
    let mut map: HashMap<&'static mime::Mime, Correspondent<Syntax>> = HashMap::new();
    set_correspondence!(
        map;
        &media_type::TEXT_HTML, syntax::HTML_RDFA, false;

        &media_type::APPLICATION_JSON_LD, syntax::JSON_LD, true;

        &media_type::TEXT_N3, syntax::N3, true;

        &media_type::APPLICATION_N_QUADS, syntax::N_QUADS, true;

        &media_type::APPLICATION_N_TRIPLES, syntax::N_TRIPLES, true;

        &media_type::TEXT_OWL_MANCHESTER, syntax::OWL2_MANCHESTER, true;

        &media_type::APPLICATION_RDF_XML, syntax::RDF_XML, true;

        &media_type::APPLICATION_OWL_XML, syntax::OWL2_XML, true;

        &media_type::APPLICATION_TRIG, syntax::TRIG, true;

        &media_type::TEXT_TURTLE, syntax::TURTLE, true;

        &media_type::APPLICATION_XHTML_XML, syntax::XHTML_RDFA, false;
    );
    map
});


#[cfg(test)]
mod tests {
    // see tests for syntax_hint module. They cover these mappings too.
}
