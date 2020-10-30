use crate::dependency::DependencyClone;
use crate::{Dependency, Resolver, GetDependencies};
use frunk::hlist::Selector;
use once_cell::sync::OnceCell;
use std::marker::PhantomData;

pub trait Init {
    type Data;
    fn init(data: Self::Data) -> Self;
}
pub trait ContainerElem<T> {}

pub struct TransientContainerElem<T>(PhantomData<T>);
impl<T> Init for TransientContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<T> ContainerElem<T> for TransientContainerElem<T> {}
impl<'a, T, SP, Deps, Index, DepsElems, Indexes>
    Resolver<'a, TransientContainerElem<T>, T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    Deps: 'a,
    SP: Selector<TransientContainerElem<T>, Index>
        + GetDependencies<'a, Deps, DepsElems, Indexes>
        + 'a,
    T: Dependency<Deps> + 'a,
{
    fn resolve(&'a self) -> T {
        T::init(self.get_deps())
    }
}

pub struct SingletonContainerElem<T>(OnceCell<T>);
impl<T> Init for SingletonContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(OnceCell::new())
    }
}
impl<T> ContainerElem<T> for SingletonContainerElem<T> {}
impl<'a, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, SingletonContainerElem<T>, T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    SP: Selector<SingletonContainerElem<T>, Index>
        + GetDependencies<'a, Deps, DepsElems, Indexes>
        + 'a,
    T: Dependency<Deps> + DependencyClone + 'a,
    Deps: 'a,
{
    fn resolve(&'a self) -> T {
        let elem = self.get();
        let elem_ref = elem.get().get();
        match elem_ref {
            None => {
                let needed = self.get_deps();
                let dep = T::init(needed);
                match elem.get().set(dep.clone()) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                dep
            }
            Some(dep) => dep.clone(),
        }
    }
}
impl<T> SingletonContainerElem<T> {
    #[inline]
    pub fn get(&self) -> &OnceCell<T> {
        &self.0
    }
}

pub struct InstanceContainerElem<T>(T);
impl<T> ContainerElem<T> for InstanceContainerElem<T> {}
impl<T> Init for InstanceContainerElem<T> {
    type Data = T;

    fn init(instance: T) -> Self {
        Self(instance)
    }
}
impl<'a, T, SP, Index> Resolver<'a, InstanceContainerElem<T>, T, SP, Index> for SP
where
    SP: Selector<InstanceContainerElem<T>, Index>,
    T: DependencyClone + 'a,
{
    fn resolve(&'a self) -> T {
        self.get().get().clone()
    }
}
impl<T> InstanceContainerElem<T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }
}

pub struct ByRefSingletonContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefInstanceContainerElem<T> {}
impl<T> Init for ByRefSingletonContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, ByRefSingletonContainerElem<T>, &'a T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    SP: Selector<SingletonContainerElem<T>, Index>
        + GetDependencies<'a, Deps, DepsElems, Indexes>
        + 'a,
    T: Dependency<Deps> + 'a,
    Deps: 'a,
{
    fn resolve(&'a self) -> &'a T {
        let elem = self.get();
        let elem_ref = elem.get().get();
        match elem_ref {
            None => {
                let needed = self.get_deps();
                let dep = T::init(needed);
                match elem.get().set(dep) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                elem.get().get().expect("Should never been failed")
            }
            Some(dep) => dep,
        }
    }
}

pub struct ByRefInstanceContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefSingletonContainerElem<T> {}
impl<T> Init for ByRefInstanceContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T, SP, Index> Resolver<'a, ByRefInstanceContainerElem<T>, &'a T, SP, Index> for SP
where
    SP: Selector<InstanceContainerElem<T>, Index> + 'a,
{
    fn resolve(&'a self) -> &'a T {
        self.get().get()
    }
}

pub struct ConvertContainerElem<CE, CET, T>(CE, PhantomData<(CET, T)>);
impl<CE, CET, T> ContainerElem<&T> for ConvertContainerElem<CE, CET, T> {}
impl<CE, CET, T> Init for ConvertContainerElem<CE, CET, T>
where
    CE: Init,
{
    type Data = CE::Data;

    fn init(data: Self::Data) -> Self {
        Self(CE::init(data), PhantomData)
    }
} /*
  impl<'a, CE, CET, T, SP, Index, Other> ContainerElem<'a, T, SP, Index, Other> for ConvertContainerElem<CE, CET, T>
  where
      CE: ContainerElem<'a, CET, SP, Index, Other>,
      CET: Into<T>,
  {
      fn resolve(service_provider: &'a SP) -> T {
          CE::resolve(service_provider).into()
      }
  }*/
impl<Cont, ContT, T> ConvertContainerElem<Cont, ContT, T> {
    #[inline]
    pub fn get(&self) -> &Cont {
        &self.0
    }
}
