use std::{borrow::Cow, ffi::OsStr, fmt::Display, ops::Deref, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A struct that denotes a file extension
pub struct FileExtension(pub Cow<'static, str>);

impl Deref for FileExtension {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for FileExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for FileExtension {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl FileExtension {
    pub fn from_path(path: &Path) -> Option<Self> {
        Some(Self::from(
            path.extension().and_then(OsStr::to_str)?.to_string(),
        ))
    }

    pub fn from_path_str(path_str: &str) -> Option<Self> {
        Self::from_path(Path::new(path_str))
    }
}

pub const HTML: FileExtension = FileExtension(Cow::Borrowed("html"));

pub const JSON: FileExtension = FileExtension(Cow::Borrowed("json"));

pub const JSONLD: FileExtension = FileExtension(Cow::Borrowed("jsonld"));

pub const N3: FileExtension = FileExtension(Cow::Borrowed("n3"));

pub const NQ: FileExtension = FileExtension(Cow::Borrowed("nq"));

pub const NQUADS: FileExtension = FileExtension(Cow::Borrowed("nquads"));

pub const NT: FileExtension = FileExtension(Cow::Borrowed("nt"));

pub const NTRIPLES: FileExtension = FileExtension(Cow::Borrowed("ttl"));

pub const OMN: FileExtension = FileExtension(Cow::Borrowed("omn"));

pub const OWL: FileExtension = FileExtension(Cow::Borrowed("owl"));

pub const OWX: FileExtension = FileExtension(Cow::Borrowed("owx"));

pub const RDF: FileExtension = FileExtension(Cow::Borrowed("rdf"));

pub const RDFXML: FileExtension = FileExtension(Cow::Borrowed("rdfxml"));

pub const TRIG: FileExtension = FileExtension(Cow::Borrowed("trig"));

pub const TTL: FileExtension = FileExtension(Cow::Borrowed("ttl"));

pub const TURTLE: FileExtension = FileExtension(Cow::Borrowed("turtle"));

pub const XHTML: FileExtension = FileExtension(Cow::Borrowed("xhtml"));
