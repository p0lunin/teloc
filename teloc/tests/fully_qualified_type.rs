use teloc::dev::container::{ConvertContainer, InstanceContainer, TransientContainer};
use teloc::reexport::frunk::{HCons, HNil};
use teloc::*;
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

// This type was captured from diagnostic output
type SP = ServiceProvider<
    EmptyServiceProvider,
    HCons<
        InstanceContainer<NumberServiceOptions>,
        HCons<
            TransientContainer<Controller>,
            HCons<
                ConvertContainer<
                    TransientContainer<Box<ConstService>>,
                    Box<ConstService>,
                    Box<dyn NumberService>,
                >,
                HNil,
            >,
        >,
    >,
>;

#[test]
fn test() {
    let options = NumberServiceOptions(10);
    let container: SP = ServiceProvider::new()
        .add_transient_c::<Box<dyn NumberService>, Box<ConstService>>()
        .add_transient::<Controller>()
        .add_instance(options);
    let controller: Controller = container.resolve();

    assert_eq!(controller.number_service.get_num(), 10);
}
