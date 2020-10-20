use teloc::{container, Get, Teloc};

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
    number_service: ConstService,
}

#[test]
fn test() {
    let service = ConstService::new(10);
    let mut container = container![ConstService = service, Controller];
    let controller: Controller = container.get();
    assert_eq!(controller.number_service.number, 10);
}
