use teloc::*;

struct ConstServiceConfig {
    config_value: i32,
}

// Declare your structs
struct ConstService<'a> {
    config: &'a ConstServiceConfig,
}

// #[inject] macro is indicate that dependency can be constructed using this
// function
#[inject]
impl<'a> ConstService<'a> {
    pub fn new(config: &'a ConstServiceConfig) -> Self {
        ConstService { config }
    }
}

// Derive macro can be used when all fields implement `Dependency` trait, but
// we recommend using the #[inject] macro it in production code instead.
#[derive(Dependency)]
struct Controller<'a> {
    number_service: &'a ConstService<'a>,
}

#[test]
fn test() {
    // Create `ServiceProvider` struct that store itself all dependencies
    let sp = ServiceProvider::new()
        // Add dependency with `Singleton` lifetime. More about lifetimes see above.
        .add_singleton::<ConstService>()
        // Add dependency with `Transient` lifetime. More about lifetimes see above.
        .add_transient::<Controller>();
    // Fork `ServiceProvider`. It creates a new `ServiceProvider` which will have
    // access to the dependencies from parent `ServiceProvider`.
    let scope = sp
        // .fork() method creates a local mutable scope with self parent immutable `ServiceProvider`.
        .fork()
        // Add an instance of `i32` that will be used when `ConstService` will be initialized.
        .add_instance(ConstServiceConfig { config_value: 10 });
    // Get dependency from `ServiceProvider`
    let controller: Controller = scope.resolve(); // fails here on resolve()
    assert_eq!(controller.number_service.config.config_value, 10);
}
