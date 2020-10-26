use std::rc::Rc;
use teloc::{Container, Get, Teloc};

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
struct Controller {
    #[init(0, 1)]
    service: ConstService,
}

#[derive(Teloc)]
struct Schema1Cloned {
    #[by(clone)]
    a: Rc<Controller>,
}

#[derive(Teloc)]
struct Schema2Cloned {
    #[by(clone)]
    a: Rc<Controller>,
}

#[test]
fn test_cloned() {
    let mut container = Container::new()
        .add::<Rc<Controller>, _>()
        .add::<Schema1Cloned, _>()
        .add::<Schema2Cloned, _>();

    let schema1: Schema1Cloned = container.get();
    assert_eq!(schema1.a.service.data, 0);
    assert_eq!(schema1.a.service.data2, 1);

    let schema2: Schema2Cloned = container.get();
    assert_eq!(schema2.a.service.data, 0);
    assert_eq!(schema2.a.service.data2, 1);
}
