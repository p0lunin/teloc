use teloc::{inject, Resolver, ServiceProvider};

struct ConstService {
    number: u8,
}

#[inject]
impl ConstService {
    #[inject::init]
    pub fn new(number: &u8) -> Self {
        Self { number: *number }
    }

    pub fn _ignore(_number: &u8) -> Self {
        Self { number: 0 }
    }
}

#[test]
fn test() {
    let provider = ServiceProvider::new()
        .add_instance(10u8)
        .add_transient::<ConstService>();

    let service: ConstService = provider.resolve();
    assert_eq!(service.number, 10u8);
}
