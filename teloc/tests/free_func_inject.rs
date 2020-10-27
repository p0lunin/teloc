use teloc::{inject, Get, Hlist, ServiceProvider, Teloc};

struct NumberServiceOptions(i32);

struct ConstService {
    number: i32,
}

#[inject]
fn init_const_service(options: &NumberServiceOptions) -> ConstService {
    ConstService { number: options.0 }
}

#[derive(Teloc)]
struct Controller {
    number_service: ConstService,
}

#[test]
fn test() {
    let options = NumberServiceOptions(10);
    let container = ServiceProvider::new()
        .add_instance(options)
        .add_transient::<ConstService>()
        .add_transient::<Controller>();
    let controller: Controller = container.get();

    assert_eq!(controller.number_service.number, 10);
}
