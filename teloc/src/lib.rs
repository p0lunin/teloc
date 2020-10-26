pub use frunk;
use frunk::hlist::{HList, Selector};
use frunk::{HCons, HNil};
use once_cell::sync::OnceCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
pub use teloc_macros::Teloc;

pub trait Get<'a, T: ContainerElem<TE>, TE, Index>
where
    TE: 'a,
{
    fn get(&'a self) -> TE;
}

pub trait Dependency<Deps> {
    fn init(deps: Deps) -> Self;
}

impl<Deps, D> Dependency<Deps> for Rc<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        Rc::new(D::init(deps))
    }
}
impl<Deps, D> Dependency<Deps> for Box<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        Box::new(D::init(deps))
    }
}
impl<Deps, D> Dependency<Deps> for Arc<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        Arc::new(D::init(deps))
    }
}

pub trait ContainerElem<Elem> {
    type Data;
    fn init(data: Self::Data) -> Self;
}
/*
impl<Elem, T> ContainerElem<Elem> for OnceCell<T> where T: ContainerElem<Elem> {
    type Data = T::Data;

    fn init(data: Self::Data) -> Self {
        let cell = OnceCell::new();
        cell.set(T::init(data));
        cell
    }
}*/

pub struct TransientContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<T> for TransientContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}

pub struct SingletonContainerElem<T>(OnceCell<T>);
impl<T> ContainerElem<T> for SingletonContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(OnceCell::new())
    }
}

pub struct InstanceContainerElem<T>(T);
impl<T> ContainerElem<T> for InstanceContainerElem<T> {
    type Data = T;

    fn init(instance: T) -> Self {
        Self(instance)
    }
}

pub struct ByRefSingletonContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefSingletonContainerElem<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(PhantomData)
    }
}

pub struct ByRefInstanceContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<&T> for ByRefInstanceContainerElem<T> {
    type Data = ();

    fn init(_: Self::Data) -> Self {
        Self(PhantomData)
    }
}

pub struct Container<Dependencies = HNil> {
    dependencies: Dependencies,
}

impl Container {
    pub fn new() -> Self {
        Container { dependencies: HNil }
    }
}

impl<H: HList> Container<H> {
    pub fn add<TE, T: ContainerElem<TE>>(self, data: T::Data) -> Container<HCons<T, H>> {
        let Container { dependencies } = self;
        Container {
            dependencies: dependencies.prepend(T::init(data)),
        }
    }
    pub fn add_transient<T>(self) -> Container<HCons<TransientContainerElem<T>, H>> {
        self.add::<T, TransientContainerElem<T>>(())
    }
    pub fn add_singleton<T>(self) -> Container<HCons<SingletonContainerElem<T>, H>> {
        self.add::<T, SingletonContainerElem<T>>(())
    }
    pub fn add_instance<T>(self, data: T) -> Container<HCons<InstanceContainerElem<T>, H>> {
        self.add::<T, InstanceContainerElem<T>>(data)
    }
}

impl<'a, H, T, Index, Deps, DepsElems, Indexes>
    Get<'a, TransientContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Container<H>
where
    H: Selector<TransientContainerElem<T>, Index>,
    T: Dependency<Deps> + 'a,
    Deps: 'a,
    Container<H>: GetDependencies<'a, Deps, DepsElems, Indexes>,
{
    fn get(&'a self) -> T {
        let res = T::init(self.get_deps());
        res
    }
}

impl<'a, H, T, Index, Deps, DepsElems, Indexes>
    Get<'a, SingletonContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Container<H>
where
    H: Selector<SingletonContainerElem<T>, Index>,
    T: Dependency<Deps> + Clone + 'a,
    Deps: 'a,
    Container<H>: GetDependencies<'a, Deps, DepsElems, Indexes>,
{
    fn get(&'a self) -> T {
        let Container { dependencies } = &self;

        let elem = dependencies.get();
        let elem_ref = elem.0.get();
        match elem_ref {
            None => {
                let needed = self.get_deps();
                let dep = T::init(needed);
                match elem.0.set(dep.clone()) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                dep
            }
            Some(dep) => dep.clone(),
        }
    }
}
impl<'a, H, T, Index, Deps, DepsElems, Indexes>
    Get<'a, ByRefSingletonContainerElem<T>, &'a T, (Index, Deps, DepsElems, Indexes)>
    for Container<H>
where
    H: Selector<SingletonContainerElem<T>, Index>,
    T: Dependency<Deps> + Clone + 'a,
    Deps: 'a,
    Container<H>: GetDependencies<'a, Deps, DepsElems, Indexes>,
{
    fn get(&'a self) -> &'a T {
        let Container { dependencies } = &self;

        let elem = dependencies.get();
        let elem_ref = elem.0.get();
        match elem_ref {
            None => {
                let needed = self.get_deps();
                let dep = T::init(needed);
                match elem.0.set(dep) {
                    Ok(()) => {}
                    Err(_) => unreachable!("Should never been reached"),
                }
                elem.0.get().expect("Should never been failed")
            }
            Some(dep) => dep,
        }
    }
}

impl<'a, H, T, Index> Get<'a, ByRefInstanceContainerElem<T>, &'a T, Index> for Container<H>
where
    H: Selector<InstanceContainerElem<T>, Index>,
{
    fn get(&'a self) -> &'a T {
        let Container { dependencies } = &self;
        let elem = dependencies.get();
        &elem.0
    }
}

impl<'a, H, T, Index> Get<'a, InstanceContainerElem<T>, T, Index> for Container<H>
where
    H: Selector<InstanceContainerElem<T>, Index>,
    T: Clone + 'a,
{
    fn get(&'a self) -> T {
        self.dependencies.get().0.clone()
    }
}

pub trait GetDependencies<'a, Dependencies: 'a, DepElems, Indexes> {
    fn get_deps(&'a self) -> Dependencies;
}

impl<'a, T, TE, TER, TR, H, I, IR> GetDependencies<'a, HCons<TE, TER>, HCons<T, TR>, HCons<I, IR>>
    for Container<H>
where
    TER: HList,
    T: ContainerElem<TE>,
    TE: 'a,
    TER: 'a,
    Container<H>: Get<'a, T, TE, I> + GetDependencies<'a, TER, TR, IR>,
{
    fn get_deps(&'a self) -> HCons<TE, TER> {
        GetDependencies::<TER, TR, IR>::get_deps(self).prepend(self.get())
    }
}
impl<'a, H> GetDependencies<'a, HNil, HNil, HNil> for Container<H> {
    fn get_deps(&'a self) -> HNil {
        HNil
    }
}

#[macro_export]
macro_rules! HList {
    [] => { teloc::frunk::HNil };
    [$x:ty] => { teloc::frunk::HCons<$x, teloc::HList![]> };
    [$x:ty, $($xs:ty),+] => { teloc::frunk::hlist::HCons<$x, teloc::HList![$($xs),*]> };
}
