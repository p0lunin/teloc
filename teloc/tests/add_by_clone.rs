use std::rc::Rc;
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
struct Controller {
    #[init(0, 1)]
    service: ConstService,
}

#[derive(Dependency)]
struct Schema1Cloned {
    a: Rc<Controller>,
}

#[derive(Dependency)]
struct Schema2Cloned {
    a: Rc<Controller>,
}

#[test]
fn test_cloned() {
    let container = ServiceProvider::new()
        .add_singleton::<Rc<Controller>>()
        .add_transient::<Schema1Cloned>()
        .add_transient::<Schema2Cloned>();

    let schema1: Schema1Cloned = container.resolve();
    assert_eq!(schema1.a.service.data, 0);
    assert_eq!(schema1.a.service.data2, 1);

    let schema2: Schema2Cloned = container.resolve();
    assert_eq!(schema2.a.service.data, 0);
    assert_eq!(schema2.a.service.data2, 1);
}
