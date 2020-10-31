use crate::container::Container;

pub trait Resolver<'a, T: Container<TE>, TE, SP, Other>
where
    TE: 'a,
{
    fn resolve(&'a self) -> TE;
}
