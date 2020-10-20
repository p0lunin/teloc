use teloc::{container, Get, Getable, Teloc};

struct NumberServiceOptions(i32);

trait NumberService {
    fn get_num(&self) -> i32;
}

struct ConstService {
    number: i32,
}
impl ConstService {
    pub fn new(number: i32) -> Self {
        ConstService { number }
    }
    fn init<T: Getable<NumberServiceOptions>, C: Get<T, NumberServiceOptions>>(
        container: &mut C,
    ) -> Self {
        let options = container.get();
        ConstService::new(options.0)
    }
}
impl NumberService for ConstService {
    fn get_num(&self) -> i32 {
        self.number
    }
}

#[derive(Teloc)]
struct Controller<N: NumberService> {
    number_service: N,
}

#[test]
fn test() {
    let options = NumberServiceOptions(10);
    let mut container = container![
        NumberServiceOptions = options,
        ConstService,
        Controller<ConstService>
    ];
    let controller: Controller<_> = container.get();

    assert_eq!(controller.number_service.get_num(), 10);
}
