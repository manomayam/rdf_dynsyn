use sophia_api::{quad::Quad, triple::TripleAsQuad};
use sophia_rio::parser::{RioSourceQuad, RioSourceTriple, RioTermWrapper};

pub(crate) enum InnerUnsafeQuad {
    RioSourceQuad(RioSourceQuad<'static>),
    RioSourceTripleAsQuad(TripleAsQuad<RioSourceTriple<'static>>),
}

pub struct SomeHowQuad(InnerUnsafeQuad);

impl Quad for SomeHowQuad {
    type Term = RioTermWrapper<'static>;

    fn s(&self) -> &Self::Term {
        match &self.0 {
            InnerUnsafeQuad::RioSourceQuad(q) => q.s(),
            InnerUnsafeQuad::RioSourceTripleAsQuad(q) => q.s(),
        }
    }

    fn p(&self) -> &Self::Term {
        match &self.0 {
            InnerUnsafeQuad::RioSourceQuad(q) => q.p(),
            InnerUnsafeQuad::RioSourceTripleAsQuad(q) => q.p(),
        }
    }

    fn o(&self) -> &Self::Term {
        match &self.0 {
            InnerUnsafeQuad::RioSourceQuad(q) => q.o(),
            InnerUnsafeQuad::RioSourceTripleAsQuad(q) => q.o(),
        }
    }

    fn g(&self) -> Option<&Self::Term> {
        match &self.0 {
            InnerUnsafeQuad::RioSourceQuad(q) => q.g(),
            InnerUnsafeQuad::RioSourceTripleAsQuad(q) => q.g(),
        }
    }
}
