use mime::Mime;
use once_cell::sync::Lazy;

pub static APPLICATION_JSON_LD: Lazy<Mime> = Lazy::new(|| "application/ld+json".parse().unwrap());

pub static APPLICATION_N_QUADS: Lazy<Mime> = Lazy::new(|| "application/n-quads".parse().unwrap());

pub static APPLICATION_N_TRIPLES: Lazy<Mime> =
    Lazy::new(|| "application/n-triples".parse().unwrap());

pub static APPLICATION_OWL_XML: Lazy<Mime> = Lazy::new(|| "application/owl+xml".parse().unwrap());

pub static APPLICATION_RDF_XML: Lazy<Mime> = Lazy::new(|| "application/rdf+xml".parse().unwrap());

pub static APPLICATION_TRIG: Lazy<Mime> = Lazy::new(|| "application/trig".parse().unwrap());

pub static APPLICATION_XHTML_XML: Lazy<Mime> =
    Lazy::new(|| "application/xhtml+xml".parse().unwrap());

pub static TEXT_HTML: Lazy<Mime> = Lazy::new(|| mime::TEXT_HTML);

pub static TEXT_N3: Lazy<Mime> = Lazy::new(|| "text/n3".parse().unwrap());

pub static TEXT_OWL_MANCHESTER: Lazy<Mime> = Lazy::new(|| "text/owl-manchester".parse().unwrap());

pub static TEXT_TURTLE: Lazy<Mime> = Lazy::new(|| "text/turtle".parse().unwrap());
