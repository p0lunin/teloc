use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

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
impl<Deps, D> Dependency<Deps> for RefCell<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        RefCell::new(D::init(deps))
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

pub trait DependencyClone: Clone {}

impl<D> DependencyClone for Rc<D> {}

impl<D> DependencyClone for Arc<D> {}

impl<D> DependencyClone for &D {}
