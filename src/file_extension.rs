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

    pub const fn from_static(v: &'static str) -> Self {
        Self(Cow::Borrowed(v))
    }
}

pub const HTML: FileExtension = FileExtension::from_static("html");

pub const JSON: FileExtension = FileExtension::from_static("json");

pub const JSONLD: FileExtension = FileExtension::from_static("jsonld");

pub const N3: FileExtension = FileExtension::from_static("n3");

pub const NQ: FileExtension = FileExtension::from_static("nq");

pub const NQUADS: FileExtension = FileExtension::from_static("nquads");

pub const NT: FileExtension = FileExtension::from_static("nt");

pub const NTRIPLES: FileExtension = FileExtension::from_static("ttl");

pub const OMN: FileExtension = FileExtension::from_static("omn");

pub const OWL: FileExtension = FileExtension::from_static("owl");

pub const OWX: FileExtension = FileExtension::from_static("owx");

pub const RDF: FileExtension = FileExtension::from_static("rdf");

pub const RDFXML: FileExtension = FileExtension::from_static("rdfxml");

pub const TRIG: FileExtension = FileExtension::from_static("trig");

pub const TTL: FileExtension = FileExtension::from_static("ttl");

pub const TURTLE: FileExtension = FileExtension::from_static("turtle");

pub const XHTML: FileExtension = FileExtension::from_static("xhtml");
