use frunk::{HCons, HNil};
use frunk::hlist::HList;

pub trait Append<H> {
    type Result;
    fn append(self, other: H) -> Self::Result;
}

impl<T, Rest, H> Append<H> for HCons<T, Rest>
where
    Rest: HList + Append<H>,
    Rest::Result: HList
{
    type Result = HCons<T, Rest::Result>;

    fn append(self, other: H) -> Self::Result {
        let HCons { head, tail } = self;
        tail.append(other).prepend(head)
    }
}

impl<H> Append<H> for HNil {
    type Result = H;

    fn append(self, other: H) -> Self::Result {
        other
    }
}