use teloc::{Dependency, Resolver, ServiceProvider};

struct ConstService {
    data: i32,
    data2: u8,
}
impl ConstService {
    pub fn init(data: i32, data2: u8) -> Self {
        ConstService { data, data2 }
    }
}

#[derive(Dependency)]
struct ControllerA {
    #[init(0, 1)]
    service: ConstService,
}

#[derive(Dependency)]
struct ControllerB {
    #[init(1, 5)]
    service: ConstService,
}

#[derive(Dependency)]
struct Schema {
    a: ControllerA,
    b: ControllerB,
}

#[test]
fn test() {
    let container = ServiceProvider::new()
        .add_transient::<ControllerA>()
        .add_transient::<ControllerB>()
        .add_transient::<Schema>();
    let schema: Schema = container.resolve();
    assert_eq!(schema.a.service.data, 0);
    assert_eq!(schema.a.service.data2, 1);
    assert_eq!(schema.b.service.data, 1);
    assert_eq!(schema.b.service.data2, 5);
}
