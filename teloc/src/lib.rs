pub use frunk;
use frunk::hlist::{HList, Selector};
use frunk::{HCons, HNil};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
pub use teloc_macros::Teloc;

pub trait Get<T: ContainerElem<TE>, TE, Index> {
    fn get(&mut self) -> TE;
}
/*pub trait GetRef<T> {
    fn get_ref(&self) -> &T;
}
pub trait GetClone<T: Clone> {
    fn get_clone(&self) -> T;
}*/

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

pub struct TransientContainerElem<T>(PhantomData<T>);
impl<T> ContainerElem<T> for TransientContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(PhantomData)
    }
}

pub struct SingletonContainerElem<T>(Option<T>);
impl<T> ContainerElem<T> for SingletonContainerElem<T> {
    type Data = ();

    fn init(_: ()) -> Self {
        Self(None)
    }
}

pub struct InstanceContainerElem<T>(T);
impl<T> ContainerElem<T> for InstanceContainerElem<T> {
    type Data = T;

    fn init(instance: T) -> Self {
        Self(instance)
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

impl<H, T, Index, Deps, DepsElems, Indexes>
    Get<TransientContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Container<H>
where
    H: Selector<TransientContainerElem<T>, Index>,
    T: Dependency<Deps>,
    Container<H>: GetDependencies<Deps, DepsElems, Indexes>,
{
    fn get(&mut self) -> T {
        let res = T::init(self.get_deps());
        res
    }
}

impl<H, T, Index, Deps, DepsElems, Indexes>
    Get<SingletonContainerElem<T>, T, (Index, Deps, DepsElems, Indexes)> for Container<H>
where
    H: Selector<SingletonContainerElem<T>, Index>,
    T: Dependency<Deps> + Clone,
    Container<H>: GetDependencies<Deps, DepsElems, Indexes>,
{
    fn get(&mut self) -> T {
        let Container { dependencies } = &self;

        match &dependencies.get().0 {
            None => {
                let needed = self.get_deps();
                let dep = T::init(needed);
                let Container { dependencies } = self;
                let t = dependencies.get_mut();
                *t = SingletonContainerElem(Some(dep.clone()));
                dep
            }
            Some(dep) => dep.clone(),
        }
    }
}

impl<H, T, Index> Get<InstanceContainerElem<T>, T, Index> for Container<H>
where
    H: Selector<InstanceContainerElem<T>, Index>,
    T: Clone,
{
    fn get(&mut self) -> T {
        self.dependencies.get().0.clone()
    }
}

pub trait GetDependencies<Dependencies, DepElems, Indexes> {
    fn get_deps(&mut self) -> Dependencies;
}

impl<T, TE, TER, TR, H, I, IR> GetDependencies<HCons<TE, TER>, HCons<T, TR>, HCons<I, IR>>
    for Container<H>
where
    TER: HList,
    T: ContainerElem<TE>,
    Container<H>: Get<T, TE, I> + GetDependencies<TER, TR, IR>,
{
    fn get_deps(&mut self) -> HCons<TE, TER> {
        GetDependencies::<TER, TR, IR>::get_deps(self).prepend(self.get())
    }
}
impl<H> GetDependencies<HNil, HNil, HNil> for Container<H> {
    fn get_deps(&mut self) -> HNil {
        HNil
    }
}

#[macro_export]
macro_rules! HList {
    [] => { teloc::frunk::HNil };
    [$x:ty] => { teloc::frunk::HCons<$x, teloc::HList![]> };
    [$x:ty, $($xs:ty),+] => { teloc::frunk::hlist::HCons<$x, teloc::HList![$($xs),*]> };
}
