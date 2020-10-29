use crate::container_elem::ContainerElem;

pub trait Get<'a, T: ContainerElem<TE>, TE, SP, Other>
where
    TE: 'a,
{
    fn resolve(&'a self) -> TE;
}
