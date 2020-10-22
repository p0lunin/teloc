pub use frunk;
use frunk::hlist::{HList, Selector};
use frunk::{HCons, HNil};
use std::rc::Rc;
use std::sync::Arc;
pub use teloc_macros::Teloc;

pub trait Get<T, Index> {
    fn get(&mut self) -> T;
}
pub trait GetRef<T, Index> {
    fn get_ref(&self) -> &T;
}
pub trait GetClone<T: Clone, Index> {
    fn get_clone(&self) -> T;
}

pub trait Dependency<Deps, Indices> {
    fn init(container: &mut Container<Deps>) -> Self;
}

impl<Deps, Indices, D> Dependency<Deps, Indices> for Rc<D>
where
    D: Dependency<Deps, Indices>,
{
    fn init(container: &mut Container<Deps>) -> Self {
        Rc::new(D::init(container))
    }
}
impl<Deps, Indices, D> Dependency<Deps, Indices> for Box<D>
where
    D: Dependency<Deps, Indices>,
{
    fn init(container: &mut Container<Deps>) -> Self {
        Box::new(D::init(container))
    }
}
impl<Deps, Indices, D> Dependency<Deps, Indices> for Arc<D>
where
    D: Dependency<Deps, Indices>,
{
    fn init(container: &mut Container<Deps>) -> Self {
        Arc::new(D::init(container))
    }
}

pub struct Container<H>(H);

impl Container<HNil> {
    pub fn new() -> Self {
        Container(HNil)
    }
}

impl<H: HList> Container<H> {
    pub fn add<T, Idxs>(self) -> Container<HCons<Option<T>, H>>
    where
        T: Dependency<H, Idxs>,
    {
        self.add_interface::<T, T, Idxs>()
    }
    pub fn add_instance<T>(self, instance: T) -> Container<HCons<Option<T>, H>> {
        let Container(depths) = self;
        Container(depths.prepend(Some(instance)))
    }
    pub fn add_interface<I, T, Idxs>(mut self) -> Container<HCons<Option<I>, H>>
    where
        T: Into<I> + Dependency<H, Idxs>,
    {
        let depth = T::init(&mut self);
        let Container(depths) = self;
        Container(depths.prepend(Some(depth.into())))
    }
}

impl<T, H, Index> Get<T, Index> for Container<H>
where
    H: Selector<Option<T>, Index>,
{
    fn get(&mut self) -> T {
        let Container(dependencies) = self;
        let t = dependencies.get_mut();
        let res = std::mem::take(t);
        res.unwrap()
    }
}

impl<T, H: Selector<Option<T>, Index>, Index> GetRef<T, Index> for Container<H> {
    fn get_ref(&self) -> &T {
        self.0.get().as_ref().unwrap()
    }
}

impl<T, H, Index> GetClone<T, Index> for Container<H>
where
    T: Clone,
    Container<H>: GetRef<T, Index>,
{
    fn get_clone(&self) -> T {
        self.get_ref().clone()
    }
}

#[macro_export]
macro_rules! HList {
    [] => { teloc::frunk::HNil };
    [$x:ty] => { teloc::frunk::HCons<$x, teloc::HList![]> };
    [$x:ty, $($xs:ty),+] => { teloc::frunk::hlist::HCons<$x, teloc::HList![$($xs),*]> };
}
