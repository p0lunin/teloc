use std::rc::Rc;
use teloc::{Get, ServiceProvider, Teloc};

struct ConstService {
    number: i32,
}
impl ConstService {
    pub fn new(number: i32) -> Self {
        ConstService { number }
    }
}

#[derive(Teloc)]
struct Controller {
    number_service: Rc<ConstService>,
}

#[test]
fn test() {
    let service = Rc::new(ConstService::new(10));
    let container = ServiceProvider::new()
        .add_instance(service)
        .add_transient::<Controller>();
    let controller: Controller = container.resolve();
    assert_eq!(controller.number_service.number, 10);
}
