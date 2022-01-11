use sophia_api::{quad::Quad, triple::TripleAsQuad};
use sophia_rio::parser::{RioSourceQuad, RioSourceTriple, RioTermWrapper};

pub(crate) enum InnerUnsafeQuad {
    Direct(RioSourceQuad<'static>),
    Wrapped(TripleAsQuad<RioSourceTriple<'static>>),
}

pub struct SomeHowQuad(InnerUnsafeQuad);

impl Quad for SomeHowQuad {
    type Term = RioTermWrapper<'static>;

    fn s(&self) -> &Self::Term {
        match &self.0 {
            InnerUnsafeQuad::Direct(q) => q.s(),
            InnerUnsafeQuad::Wrapped(q) => q.s(),
        }
    }

    fn p(&self) -> &Self::Term {
        match &self.0 {
            InnerUnsafeQuad::Direct(q) => q.p(),
            InnerUnsafeQuad::Wrapped(q) => q.p(),
        }
    }

    fn o(&self) -> &Self::Term {
        match &self.0 {
            InnerUnsafeQuad::Direct(q) => q.o(),
            InnerUnsafeQuad::Wrapped(q) => q.o(),
        }
    }

    fn g(&self) -> Option<&Self::Term> {
        match &self.0 {
            InnerUnsafeQuad::Direct(q) => q.g(),
            InnerUnsafeQuad::Wrapped(q) => q.g(),
        }
    }
}
