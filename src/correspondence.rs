use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::{
    file_extension::{self as fextn, FileExtension},
    media_type,
    syntax::{self, RdfSyntax},
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
pub static SYNTAX_TO_EXTENSION_CORRESPONDENCE: Lazy<
    HashMap<RdfSyntax, Correspondent<FileExtension>>,
> = Lazy::new(|| {
    let mut map: HashMap<RdfSyntax, Correspondent<FileExtension>> = HashMap::new();
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
pub static EXTENSION_TO_SYNTAX_CORRESPONDENCE: Lazy<
    HashMap<FileExtension, Correspondent<RdfSyntax>>,
> = Lazy::new(|| {
    let mut map: HashMap<FileExtension, Correspondent<RdfSyntax>> = HashMap::new();
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
    HashMap<RdfSyntax, Correspondent<&'static mime::Mime>>,
> = Lazy::new(|| {
    let mut map: HashMap<RdfSyntax, Correspondent<&'static mime::Mime>> = HashMap::new();
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
    HashMap<&'static mime::Mime, Correspondent<RdfSyntax>>,
> = Lazy::new(|| {
    let mut map: HashMap<&'static mime::Mime, Correspondent<RdfSyntax>> = HashMap::new();
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

/// An error of a media-type being not having any corresponding rdf syntax
#[derive(Debug, thiserror::Error, Clone)]
#[error("Specified media type {0} doesn't correspond to any rdf syntax")]
pub struct NonRdfMediaTypeError(mime::Mime);

/// An error of a file-extension being not having any corresponding rdf syntax
#[derive(Debug, thiserror::Error, Clone)]
#[error("Specified file-extension {0} doesn't correspond to any rdf syntax")]
pub struct NonRdfFileExtensionError(FileExtension);

impl TryFrom<&mime::Mime> for Correspondent<RdfSyntax> {
    type Error = NonRdfMediaTypeError;

    /// For given SyntaxHint, tries to resolve corresponding syntax.
    #[tracing::instrument(
        name = "Resolving Syntax from media type",
        fields(media_type=%media_type)
    )]
    fn try_from(media_type: &mime::Mime) -> Result<Self, Self::Error> {
        match MEDIA_TYPE_TO_SYNTAX_CORRESPONDENCE.get(media_type) {
            Some(correspondent_syntax) => {
                tracing::info!("media_type resolved to {}", &correspondent_syntax.value);
                Ok(correspondent_syntax.clone())
            }
            None => {
                tracing::error!("media_type cannot be resolved");
                Err(NonRdfMediaTypeError(media_type.clone()))
            }
        }
    }
}

impl TryFrom<&FileExtension> for Correspondent<RdfSyntax> {
    type Error = NonRdfFileExtensionError;

    /// For given SyntaxHint, tries to resolve corresponding syntax.
    #[tracing::instrument(
        name = "Resolving Syntax from file extension",
        fields(file_extension=%file_extension)
    )]
    fn try_from(file_extension: &FileExtension) -> Result<Self, Self::Error> {
        match EXTENSION_TO_SYNTAX_CORRESPONDENCE.get(file_extension) {
            Some(correspondent_syntax) => {
                tracing::info!("file_extension resolved to {}", &correspondent_syntax.value);
                Ok(correspondent_syntax.clone())
            }
            None => {
                tracing::error!("file_extension cannot be resolved");
                Err(NonRdfFileExtensionError(file_extension.clone()))
            }
        }
    }
}

// ---------------------------------------------------------------------------------
//                                      tests
// ---------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use claim::{assert_err, assert_ok};
    use once_cell::sync::Lazy;
    use test_case::test_case;

    use crate::{
        correspondence::Correspondent,
        file_extension::{self, FileExtension},
        media_type,
        syntax::RdfSyntax,
        tests::TRACING,
    };

    #[test_case("png")]
    #[test_case("pdf")]
    #[test_case("mp3")]
    #[test_case("avf")]
    #[test_case("c")]
    #[test_case("rs")]
    pub fn non_rdf_file_extensions_should_not_have_correspondent_syntax(extn_str: &'static str) {
        Lazy::force(&TRACING);
        let extn = FileExtension::from(extn_str);
        assert_err!(Correspondent::<RdfSyntax>::try_from(&extn));
    }

    #[test_case(&file_extension::HTML)]
    #[test_case(&file_extension::JSON)]
    #[test_case(&file_extension::JSONLD)]
    #[test_case(&file_extension::NQ)]
    #[test_case(&file_extension::NQUADS)]
    #[test_case(&file_extension::NT)]
    #[test_case(&file_extension::NTRIPLES)]
    #[test_case(&file_extension::OMN)]
    #[test_case(&file_extension::OWL)]
    #[test_case(&file_extension::OWX)]
    #[test_case(&file_extension::RDF)]
    #[test_case(&file_extension::RDFXML)]
    #[test_case(&file_extension::TRIG)]
    #[test_case(&file_extension::TTL)]
    #[test_case(&file_extension::TURTLE)]
    #[test_case(&file_extension::XHTML)]
    pub fn known_rdf_file_extensions_should_have_correspondent_syntax(extn: &FileExtension) {
        Lazy::force(&TRACING);
        assert_ok!(Correspondent::<RdfSyntax>::try_from(extn));
    }

    // For rdfa+html
    #[test_case(&file_extension::HTML)]
    // For json-ld
    #[test_case(&file_extension::JSON)]
    // for rdfa+xhtml
    #[test_case(&file_extension::XHTML)]
    pub fn known_general_file_extensions_should_have_non_total_correspondence(
        extn: &FileExtension,
    ) {
        Lazy::force(&TRACING);
        assert!(!Correspondent::<RdfSyntax>::try_from(extn).unwrap().is_total);
    }

    #[test_case(&file_extension::JSONLD)]
    #[test_case(&file_extension::NQ)]
    #[test_case(&file_extension::NQUADS)]
    #[test_case(&file_extension::NT)]
    #[test_case(&file_extension::NTRIPLES)]
    #[test_case(&file_extension::OMN)]
    #[test_case(&file_extension::OWL)]
    #[test_case(&file_extension::OWX)]
    #[test_case(&file_extension::RDF)]
    #[test_case(&file_extension::RDFXML)]
    #[test_case(&file_extension::TRIG)]
    #[test_case(&file_extension::TTL)]
    #[test_case(&file_extension::TURTLE)]
    pub fn known_rdf_specific_file_extensions_should_have_total_correspondence(
        extn: &FileExtension,
    ) {
        Lazy::force(&TRACING);
        assert!(Correspondent::<RdfSyntax>::try_from(extn).unwrap().is_total);
    }

    #[test_case(&mime::APPLICATION_PDF)]
    #[test_case(&mime::APPLICATION_JAVASCRIPT)]
    #[test_case(&mime::FONT_WOFF)]
    #[test_case(&mime::IMAGE_STAR)]
    #[test_case(&mime::TEXT_CSV)]
    pub fn non_rdf_media_types_should_not_have_correspondent_syntax(media_type: &mime::Mime) {
        Lazy::force(&TRACING);
        assert_err!(Correspondent::<RdfSyntax>::try_from(media_type));
    }

    #[test_case(&media_type::APPLICATION_JSON_LD)]
    #[test_case(&media_type::APPLICATION_N_QUADS)]
    #[test_case(&media_type::APPLICATION_N_TRIPLES)]
    #[test_case(&media_type::APPLICATION_OWL_XML)]
    #[test_case(&media_type::APPLICATION_RDF_XML)]
    #[test_case(&media_type::APPLICATION_TRIG)]
    #[test_case(&media_type::APPLICATION_XHTML_XML)]
    #[test_case(&media_type::TEXT_HTML)]
    #[test_case(&media_type::TEXT_N3)]
    #[test_case(&media_type::TEXT_OWL_MANCHESTER)]
    #[test_case(&media_type::TEXT_TURTLE)]
    pub fn known_rdf_media_types_should_have_correspondent_syntax(media_type: &mime::Mime) {
        Lazy::force(&TRACING);
        assert_ok!(Correspondent::<RdfSyntax>::try_from(media_type));
    }

    // For rdfa+xhtml
    #[test_case(&media_type::APPLICATION_XHTML_XML)]
    // For rdfa + html
    #[test_case(&media_type::TEXT_HTML)]
    pub fn known_general_media_types_should_have_non_total_correspondence(media_type: &mime::Mime) {
        Lazy::force(&TRACING);
        assert!(
            !Correspondent::<RdfSyntax>::try_from(media_type)
                .unwrap()
                .is_total
        );
    }

    #[test_case(&media_type::APPLICATION_JSON_LD)]
    #[test_case(&media_type::APPLICATION_N_QUADS)]
    #[test_case(&media_type::APPLICATION_N_TRIPLES)]
    #[test_case(&media_type::APPLICATION_OWL_XML)]
    #[test_case(&media_type::APPLICATION_RDF_XML)]
    #[test_case(&media_type::APPLICATION_TRIG)]
    #[test_case(&media_type::TEXT_N3)]
    #[test_case(&media_type::TEXT_OWL_MANCHESTER)]
    #[test_case(&media_type::TEXT_TURTLE)]
    pub fn known_rdf_specific_media_types_should_have_total_correspondence(
        media_type: &mime::Mime,
    ) {
        Lazy::force(&TRACING);
        assert!(
            Correspondent::<RdfSyntax>::try_from(media_type)
                .unwrap()
                .is_total
        );
    }
}
