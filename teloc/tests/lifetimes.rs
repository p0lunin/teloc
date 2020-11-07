use teloc::{inject, Dependency, Resolver, ServiceProvider};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
struct UUID(Uuid);
#[inject]
fn create_uuid() -> UUID {
    UUID(Uuid::new_v4())
}

#[derive(Dependency)]
struct Transient {
    u: UUID,
}
#[derive(Dependency)]
struct Scoped {
    u: UUID,
}
#[derive(Dependency)]
struct Singleton {
    u: UUID,
}
#[derive(Dependency)]
struct Instance {
    u: UUID,
}

#[test]
fn test_lifetimes() {
    let provider = ServiceProvider::new()
        .add_transient::<UUID>()
        .add_transient::<Transient>()
        .add_scoped::<Scoped>()
        .add_singleton::<Singleton>()
        .add_instance(Instance {
            u: UUID::init(frunk::hlist![]),
        });

    let scope1 = provider.scope(teloc::scopei![]);

    let t1: Transient = scope1.resolve();
    let t1_1: Transient = scope1.resolve();
    let sc1: &Scoped = scope1.resolve();
    let sc1_1: &Scoped = scope1.resolve();
    let si1: &Singleton = scope1.resolve();
    let i1: &Instance = scope1.resolve();

    let scope2 = provider.scope(teloc::scopei![]);

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
