use teloc::{container, Get, Getable, Teloc};

struct NumberServiceOptions(i32);

trait NumberService {
    fn get_num(&self) -> i32;
}
trait InitNumberService {
    fn init<T: Getable<NumberServiceOptions>, C: Get<T, NumberServiceOptions>>(
        container: &mut C,
    ) -> Self
    where
        Self: Sized;
}

struct ConstService {
    number: i32,
}
impl ConstService {
    pub fn new(number: i32) -> Self {
        ConstService { number }
    }
}
impl NumberService for ConstService {
    fn get_num(&self) -> i32 {
        self.number
    }
}
impl InitNumberService for Box<ConstService> {
    fn init<T: Getable<NumberServiceOptions>, C: Get<T, NumberServiceOptions>>(
        container: &mut C,
    ) -> Self {
        let options = container.get();
        Box::new(ConstService::new(options.0))
    }
}

#[derive(Teloc)]
struct Controller {
    number_service: Box<dyn NumberService>,
}

#[test]
fn test() {
    let options = NumberServiceOptions(10);
    let mut container = container! [
        NumberServiceOptions = options,
        Box<ConstService> as Box<dyn NumberService>,
        Controller
    ];
    let controller: Controller = container.get();

    assert_eq!(controller.number_service.get_num(), 10);
}
