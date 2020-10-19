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