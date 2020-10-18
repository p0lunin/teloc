use std::rc::Rc;
use teloc::{container, Get, Teloc};

struct ConstService {
    data: i32,
    data2: u8,
}
impl ConstService {
    pub fn init(data: i32, data2: u8) -> Self {
        ConstService { data, data2 }
    }
}
#[derive(Teloc)]
struct ControllerA {
    #[init(0, 1)]
    service: ConstService,
}
#[derive(Teloc)]
struct ControllerB {
    #[init(1, 5)]
    service: ConstService,
}
#[derive(Teloc)]
struct Schema {
    a: ControllerA,
    b: ControllerB,
}
#[test]
fn test() {
    let mut container = container![ControllerA, ControllerB, Schema];
    let schema: Schema = container.get();
    assert_eq!(schema.a.service.data, 0);
    assert_eq!(schema.a.service.data2, 1);
    assert_eq!(schema.b.service.data, 1);
    assert_eq!(schema.b.service.data2, 5);
}

#[derive(Teloc)]
struct Schema1Cloned {
    #[by(clone)]
    a: Rc<ControllerA>,
}

#[derive(Teloc)]
struct Schema2Cloned {
    #[by(clone)]
    a: Rc<ControllerA>,
}

#[test]
fn test_cloned() {
    let mut container = container![
        Rc<ControllerA>,
        Schema1Cloned,
        Schema2Cloned
    ];

    let schema1: Schema1Cloned = container.get();
    assert_eq!(schema1.a.service.data, 0);
    assert_eq!(schema1.a.service.data2, 1);

    let schema2: Schema2Cloned = container.get();
    assert_eq!(schema2.a.service.data, 0);
    assert_eq!(schema2.a.service.data2, 1);
}
