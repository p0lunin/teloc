use teloc::{inject, Dependency, Resolver, ServiceProvider};

#[derive(Debug, PartialEq)]
struct Uuid(uuid::Uuid);
#[inject]
fn create_uuid() -> Uuid {
    Uuid(uuid::Uuid::new_v4())
}

#[derive(Dependency)]
struct Transient {
    u: Uuid,
}
#[derive(Dependency)]
struct Scoped {
    u: Uuid,
}
#[derive(Dependency)]
struct Singleton {
    u: Uuid,
}
#[derive(Dependency)]
struct Instance {
    u: Uuid,
}

#[test]
fn test_lifetimes() {
    let provider = ServiceProvider::new()
        .add_transient::<Uuid>()
        .add_transient::<Transient>()
        .add_singleton::<Singleton>()
        .add_instance(Instance {
            u: Uuid::init(frunk::hlist![]),
        });

    let scope1 = provider.fork().add_singleton::<Scoped>();

    let t1: Transient = scope1.resolve();
    let t1_1: Transient = scope1.resolve();
    let sc1: &Scoped = scope1.resolve();
    let sc1_1: &Scoped = scope1.resolve();
    let si1: &Singleton = scope1.resolve();
    let i1: &Instance = scope1.resolve();

    let scope2 = provider.fork().add_singleton::<Scoped>();

    let t2: Transient = scope2.resolve();
    let sc2: &Scoped = scope2.resolve();
    let si2: &Singleton = scope2.resolve();
    let i2: &Instance = scope2.resolve();

    assert_ne!(t1.u, t1_1.u);
    assert_ne!(t1.u, t2.u);
    assert_ne!(sc1.u, sc2.u);

    assert_eq!(sc1.u, sc1_1.u);
    assert_eq!(si1.u, si2.u);
    assert_eq!(i1.u, i2.u);
}

#[test]
fn test_resolve_nested_fork() {
    let provider = ServiceProvider::new().add_instance(10u8);
    let forked_provider = provider.fork();
    let double_forked_provider = forked_provider.fork();

    let num: &u8 = double_forked_provider.resolve();
    assert_eq!(num, &10u8);
}

#[test]
fn test_forked_lifetime() {
    let provider = ServiceProvider::new().add_instance(10u8);
    let num: &u8 = provider.resolve();

    let forked_num: &u8 = {
        let provider = provider.fork();
        provider.resolve()
    };

    assert_eq!(num, forked_num);
}

#[test]
fn test_resolve_instance_with_greater_lifetime() {
    let num = 10u8;

    let resolved_num: &u8 = {
        let provider = ServiceProvider::new().add_instance(&num);
        provider.resolve()
    };

    assert_eq!(num, *resolved_num);
}

#[test]
fn test_resolve_singleton_with_greater_lifetime() {
    let provider = ServiceProvider::new().add_singleton::<Uuid>();
    let uuid: &Uuid = provider.resolve();

    let forked_uuid: &Uuid = {
        let provider = provider.fork();
        provider.resolve()
    };

    assert_eq!(uuid, forked_uuid);
}

#[test]
fn test_resolve_transient_with_greater_lifetime() {
    let provider = ServiceProvider::new().add_transient::<Uuid>();
    let uuid: Uuid = provider.resolve();

    let forked_uuid: Uuid = {
        let provider = provider.fork();
        provider.resolve()
    };

    assert_ne!(uuid, forked_uuid);
}

struct SingletonWithBorrowedDep<'a> {
    dep: &'a i32,
}

#[inject]
impl<'a> SingletonWithBorrowedDep<'a> {
    pub fn new(dep: &'a i32) -> Self {
        Self { dep }
    }
}

#[test]
fn test_resolve_singleton_deps_with_greater_lifetime() {
    let provider = ServiceProvider::new()
        .add_instance(10i32)
        .add_singleton::<SingletonWithBorrowedDep>();
    let singleton: &SingletonWithBorrowedDep = provider.resolve();

    let forked_singleton: &SingletonWithBorrowedDep = {
        let provider = provider.fork();
        provider.resolve()
    };

    assert_eq!(singleton.dep, forked_singleton.dep);
}

#[test]
fn test_resolve_transient_deps_with_greater_lifetime() {
    let provider = ServiceProvider::new()
        .add_instance(10i32)
        .add_transient::<SingletonWithBorrowedDep>();
    let singleton: SingletonWithBorrowedDep = provider.resolve();

    let forked_singleton: SingletonWithBorrowedDep = {
        let provider = provider.fork();
        provider.resolve()
    };

    assert_eq!(singleton.dep, forked_singleton.dep);
}
