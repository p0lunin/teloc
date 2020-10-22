use teloc::{Container, Dependency, Get, HList, Teloc};

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
}
impl<D, I1> Dependency<D, HList![I1]> for ConstService
where
    Container<D>: Get<NumberServiceOptions, I1>,
{
    fn init(container: &mut Container<D>) -> Self {
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
    let mut container = Container::new()
        .add_instance(options)
        .add::<ConstService, _>()
        .add::<Controller<ConstService>, _>();
    let controller: Controller<_> = container.get();

    assert_eq!(controller.number_service.get_num(), 10);
}
