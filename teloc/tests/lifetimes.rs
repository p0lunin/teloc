/*use uuid::Uuid;
use teloc::{Dependency, ServiceProvider};

struct UUID(Uuid);
impl Dependency<teloc::Hlist![]> for UUID {
    fn init(_: teloc::Hlist![]) -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Teloc)]
struct Transient {
    u: UUID
}
#[derive(Teloc)]
struct Scoped {
    u: UUID
}
#[derive(Teloc)]
struct Singleton {
    u: UUID
}
#[derive(Teloc)]
struct Instance {
    u: UUID
}

#[test]
fn test_lifetimes() {
    let provider = ServiceProvider::new()
        .add_transient::<UUID>()
        .add_transient::<Transient>()
        .add_scoped::<Scoped>()
        .add_singleton::<Singleton>()
        .add_instance(Instance { u: UUID::init(frunk::hlist![]) });

    let scope1 = provider.scope();

}*/
