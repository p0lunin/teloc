use crate::dependency::DependencyClone;
use crate::{Dependency, GetDependencies, Resolver};
use frunk::hlist::Selector;
use once_cell::sync::OnceCell;
use std::marker::PhantomData;

pub trait Init {
    type Data;
    fn init(data: Self::Data) -> Self;
}
pub trait Container<T> {}

pub struct TransientContainer<T>(PhantomData<T>);
impl<T> Init for TransientContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<T> Container<T> for TransientContainer<T> {}
impl<'a, T, SP, Deps, Index, DepsElems, Indexes>
    Resolver<'a, TransientContainer<T>, T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    Deps: 'a,
    SP: Selector<TransientContainer<T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes> + 'a,
    T: Dependency<Deps> + 'a,
{
    fn resolve(&'a self) -> T {
        T::init(self.get_deps())
    }
}

pub struct SingletonContainer<T>(OnceCell<T>);
impl<T> Init for SingletonContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(OnceCell::new())
    }
}
impl<T> Container<T> for SingletonContainer<T> {}
impl<'a, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, SingletonContainer<T>, T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    SP: Selector<SingletonContainer<T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes> + 'a,
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
impl<T> SingletonContainer<T> {
    #[inline]
    pub fn get(&self) -> &OnceCell<T> {
        &self.0
    }
}

pub struct InstanceContainer<T>(T);
impl<T> Container<T> for InstanceContainer<T> {}
impl<T> Init for InstanceContainer<T> {
    type Data = T;

    fn init(instance: T) -> Self {
        Self(instance)
    }
}
impl<'a, T, SP, Index> Resolver<'a, InstanceContainer<T>, T, SP, Index> for SP
where
    SP: Selector<InstanceContainer<T>, Index>,
    T: DependencyClone + 'a,
{
    fn resolve(&'a self) -> T {
        self.get().get().clone()
    }
}
impl<T> InstanceContainer<T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }
}

pub struct ByRefSingletonContainer<T>(PhantomData<T>);
impl<T> Container<&T> for ByRefInstanceContainer<T> {}
impl<T> Init for ByRefSingletonContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T, SP, Index, Deps, DepsElems, Indexes>
    Resolver<'a, ByRefSingletonContainer<T>, &'a T, SP, (Index, Deps, DepsElems, Indexes)> for SP
where
    SP: Selector<SingletonContainer<T>, Index> + GetDependencies<'a, Deps, DepsElems, Indexes> + 'a,
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

pub struct ByRefInstanceContainer<T>(PhantomData<T>);
impl<T> Container<&T> for ByRefSingletonContainer<T> {}
impl<T> Init for ByRefInstanceContainer<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}
impl<'a, T, SP, Index> Resolver<'a, ByRefInstanceContainer<T>, &'a T, SP, Index> for SP
where
    SP: Selector<InstanceContainer<T>, Index> + 'a,
{
    fn resolve(&'a self) -> &'a T {
        self.get().get()
    }
}

pub struct ConvertContainer<CE, CET, T>(CE, PhantomData<(CET, T)>);
impl<CE, CET, T> Container<&T> for ConvertContainer<CE, CET, T> {}
impl<CE, CET, T> Init for ConvertContainer<CE, CET, T>
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
impl<Cont, ContT, T> ConvertContainer<Cont, ContT, T> {
    #[inline]
    pub fn get(&self) -> &Cont {
        &self.0
    }
}
