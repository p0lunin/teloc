use teloc::{inject, Get, ServiceProvider, Teloc};

#[derive(Clone)]
struct ConstService {
    number: i32,
}

#[inject]
impl ConstService {
    pub fn new(number: i32) -> Self {
        ConstService { number }
    }
}

#[derive(Teloc)]
struct Controller {
    number_service: ConstService,
}

#[test]
fn test() {
    let container = ServiceProvider::new()
        .add_scoped::<i32>()
        .add_scoped::<bool>()
        .add_transient::<ConstService>()
        .add_transient::<Controller>();
    let scope = container.scope(frunk::hlist![true, 10]);
    let controller: Controller = scope.get();
    assert_eq!(controller.number_service.number, 10);
}
