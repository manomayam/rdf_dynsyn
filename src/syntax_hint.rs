use std::fmt::Display;

use mime::Mime;
use thiserror::Error;

use super::{
    correspondence::{
        Correspondent, EXTENSION_TO_SYNTAX_CORRESPONDENCE, MEDIA_TYPE_TO_SYNTAX_CORRESPONDENCE,
    },
    file_extension::FileExtension,
    syntax::Syntax,
};

#[derive(Debug, Clone)]
/// An enum for hinting concrete-syntax of an rdf-document
pub enum SyntaxHint<'a> {
    Syntax(&'a Syntax),
    MediaType(&'a Mime),
    FileExtension(&'a FileExtension),
}

impl Display for SyntaxHint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxHint::Syntax(v) => write!(f, "Syntax: {}", v),
            SyntaxHint::MediaType(v) => write!(f, "MediaType: {}", v),
            SyntaxHint::FileExtension(v) => write!(f, "FileExtension: {}", v),
        }
    }
}

/// An error of unknown syntax hint
#[derive(Debug, Error, Clone)]
#[error("Unknown SyntaxHint {0}")]
pub struct UnknownSyntaxHintError<'a>(SyntaxHint<'a>);

impl<'a> UnknownSyntaxHintError<'a> {
    pub fn new<T: AsRef<SyntaxHint<'a>>>(hint: &T) -> Self {
        Self(hint.as_ref().clone())
    }
}

impl<'a> TryInto<Correspondent<Syntax>> for SyntaxHint<'a> {
    type Error = UnknownSyntaxHintError<'a>;

    /// For given SyntaxHint, tries to resolve corresponding syntax.
    #[tracing::instrument(
        name = "Resolving Syntax from SyntaxHint",
        fields(hint=%self)
    )]
    fn try_into(self) -> Result<Correspondent<Syntax>, Self::Error> {
        if let SyntaxHint::Syntax(s) = self {
            tracing::info!("syntax hint resolved to {}", s);
            return Ok(Correspondent {
                value: s.clone(),
                is_total: true,
            });
        }

        let correspondent_opt = match self {
            SyntaxHint::Syntax(_) => unreachable!(),
            SyntaxHint::MediaType(mt) => MEDIA_TYPE_TO_SYNTAX_CORRESPONDENCE.get(mt),
            SyntaxHint::FileExtension(extn) => EXTENSION_TO_SYNTAX_CORRESPONDENCE.get(extn),
        };

        if let Some(correspondent) = correspondent_opt {
            tracing::info!("syntax hint resolved to {}", &correspondent.value);
            Ok(correspondent.clone())
        } else {
            tracing::error!("syntax hint cannot be resolved");
            Err(UnknownSyntaxHintError(self.clone()))
        }
    }
}

impl<'a> TryInto<Syntax> for SyntaxHint<'a> {
    type Error = UnknownSyntaxHintError<'a>;

    fn try_into(self) -> Result<Syntax, Self::Error> {
        Ok(TryInto::<Correspondent<Syntax>>::try_into(self)?
            .value
            .clone())
    }
}

pub type SyntaxHintResolutionResult<'a> = Result<Correspondent<Syntax>, UnknownSyntaxHintError<'a>>;

// ---------------------------------------------------------------------------------
//                                      tests
// ---------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};
    use once_cell::sync::Lazy;
    use test_case::test_case;

    use super::*;

    use crate::{file_extension, media_type, tests::TRACING};

    /// Asserts given syntax hint is unresolvable
    fn assert_unresolvable_hint(hint: SyntaxHint) {
        let correspondent_result: SyntaxHintResolutionResult = hint.try_into();
        assert_err!(correspondent_result);
    }

    /// Asserts given syntax hint is resolvable
    fn assert_resolvable_hint(hint: SyntaxHint) {
        let correspondent_result: SyntaxHintResolutionResult = hint.try_into();
        assert_ok!(correspondent_result);
    }

    #[test_case("png")]
    #[test_case("pdf")]
    #[test_case("mp3")]
    #[test_case("avf")]
    #[test_case("c")]
    #[test_case("rs")]
    pub fn non_rdf_extn_syntax_hints_should_be_rejected(extn_str: &'static str) {
        Lazy::force(&TRACING);
        let extn = FileExtension::from(extn_str);
        let hint = SyntaxHint::FileExtension(&extn);
        assert_unresolvable_hint(hint);
    }

    #[test_case(&mime::APPLICATION_PDF)]
    #[test_case(&mime::APPLICATION_JAVASCRIPT)]
    #[test_case(&mime::FONT_WOFF)]
    #[test_case(&mime::IMAGE_STAR)]
    #[test_case(&mime::TEXT_CSV)]
    pub fn non_rdf_media_type_syntax_hints_should_be_rejected(mt: &Mime) {
        Lazy::force(&TRACING);
        let hint = SyntaxHint::MediaType(mt);
        assert_unresolvable_hint(hint);
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
    pub fn known_rdf_extn_syntax_hints_should_be_resolved(extn: &FileExtension) {
        Lazy::force(&TRACING);
        let hint = SyntaxHint::FileExtension(extn);
        assert_resolvable_hint(hint);
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
    pub fn known_rdf_media_type_syntax_hints_should_be_resolved(mt: &Mime) {
        Lazy::force(&TRACING);
        let hint = SyntaxHint::MediaType(mt);
        assert_resolvable_hint(hint);
    }

    // For rdfa+html
    #[test_case(&file_extension::HTML)]
    // For json-ld
    #[test_case(&file_extension::JSON)]
    // for rdfa+xhtml
    #[test_case(&file_extension::XHTML)]
    pub fn general_extension_hints_should_have_non_total_correspondence(extn: &FileExtension) {
        Lazy::force(&TRACING);
        let hint = SyntaxHint::FileExtension(extn);
        let syntax_correspondent: Correspondent<Syntax> = hint.try_into().unwrap();
        assert!(!syntax_correspondent.is_total);
    }

    // For rdfa+xhtml
    #[test_case(&media_type::APPLICATION_XHTML_XML)]
    // For rdfa + html
    #[test_case(&media_type::TEXT_HTML)]
    pub fn general_media_types_should_have_non_total_correspondence(mt: &Mime) {
        Lazy::force(&TRACING);
        let hint = SyntaxHint::MediaType(mt);
        let syntax_correspondent: Correspondent<Syntax> = hint.try_into().unwrap();
        assert!(!syntax_correspondent.is_total);
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
    pub fn rdf_specific_extensions_should_have_total_correspondence(extn: &FileExtension) {
        Lazy::force(&TRACING);
        let hint = SyntaxHint::FileExtension(extn);
        let syntax_correspondent: Correspondent<Syntax> = hint.try_into().unwrap();
        assert!(syntax_correspondent.is_total);
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
    pub fn rdf_specific_media_types_should_have_total_correspondence(mt: &Mime) {
        Lazy::force(&TRACING);
        let hint = SyntaxHint::MediaType(mt);
        let syntax_correspondent: Correspondent<Syntax> = hint.try_into().unwrap();
        assert!(syntax_correspondent.is_total);
    }
}
