use teloc::{Container, Dependency, Get, HList, Teloc};

struct ConstService {
    data: i32,
    data2: u8,
}
impl ConstService {
    pub fn init(data: i32, data2: u8) -> Self {
        ConstService { data, data2 }
    }
}

struct ControllerA {
    service: ConstService,
}
impl Dependency<HList![]> for ControllerA {
    fn init(_: HList![]) -> Self {
        Self {
            service: ConstService::init(0, 1),
        }
    }
}

struct ControllerB {
    service: ConstService,
}
impl Dependency<HList![]> for ControllerB {
    fn init(_: HList![]) -> Self {
        Self {
            service: ConstService::init(1, 5),
        }
    }
}
struct Schema {
    a: ControllerA,
    b: ControllerB,
}
impl Dependency<HList![ControllerA, ControllerB]> for Schema {
    fn init(deps: HList![ControllerA, ControllerB]) -> Self {
        let (a, b) = deps.into_tuple2();
        Self { a, b }
    }
}

#[test]
fn test() {
    let mut container = Container::new()
        .add_transient::<ControllerA>()
        .add_transient::<ControllerB>()
        .add_transient::<Schema>();
    let schema: Schema = container.get();
    assert_eq!(schema.a.service.data, 0);
    assert_eq!(schema.a.service.data2, 1);
    assert_eq!(schema.b.service.data, 1);
    assert_eq!(schema.b.service.data2, 5);
}
