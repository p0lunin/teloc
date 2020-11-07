use teloc::{inject, Dependency, Resolver, ServiceProvider};

struct NumberServiceOptions(i32);

trait NumberService {
    fn get_num(&self) -> i32;
}

struct ConstService {
    number: i32,
}
impl NumberService for ConstService {
    fn get_num(&self) -> i32 {
        self.number
    }
}
#[inject]
impl ConstService {
    fn new(options: &NumberServiceOptions) -> Self {
        ConstService { number: options.0 }
    }
}
impl From<Box<ConstService>> for Box<dyn NumberService> {
    fn from(x: Box<ConstService>) -> Self {
        x
    }
}

#[derive(Dependency)]
struct Controller {
    number_service: Box<dyn NumberService>,
}

#[test]
fn test() {
    let options = NumberServiceOptions(10);
    let container = ServiceProvider::new()
        .add_instance(options)
        .add_transient_c::<Box<dyn NumberService>, Box<ConstService>>()
        .add_transient::<Controller>();
    let controller: Controller = container.resolve();

    assert_eq!(controller.number_service.get_num(), 10);
}
