use teloc::{ServiceProvider, Resolver, inject};

struct Number(i32);

struct NumberService(i32);

#[inject]
impl NumberService {
    pub fn new(number: Number) -> Self {
        Self(number.0)
    }
}

#[test]
fn test() {
    let sp = ServiceProvider::new()
        .add_transient_f(|| Number(1))
        .add_transient::<NumberService>();

    let dep: NumberService = sp.resolve();

    assert_eq!(dep.0, 1);
}