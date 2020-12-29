use std::rc::Rc;
use teloc::{Dependency, Resolver, ServiceProvider};

#[derive(Dependency)]
struct Controller;

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
    let schema2: Schema2Cloned = container.resolve();

    assert!(Rc::ptr_eq(&schema1.a, &schema2.a));
}
