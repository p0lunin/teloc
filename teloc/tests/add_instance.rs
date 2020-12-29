use std::rc::Rc;
use teloc::{Resolver, ServiceProvider};

struct ConstService;

#[test]
fn test() {
    let service = Rc::new(ConstService);
    let container = ServiceProvider::new().add_instance(service);
    let scope = container.fork();

    let first: Rc<ConstService> = container.resolve();
    let second: Rc<ConstService> = scope.resolve();

    assert!(Rc::ptr_eq(&first, &second));
}
