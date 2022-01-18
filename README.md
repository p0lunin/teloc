# teloc
<div>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://docs.rs/teloc">
    <img src="https://docs.rs/teloc/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloc">
    <img src="https://img.shields.io/crates/v/teloc.svg">
  </a>
</div>

Teloc is simple, compile-time DI framework for Rust inspired by 
<a href = "https://docs.microsoft.com/en-us/aspnet/core/fundamentals/dependency-injection?view=aspnetcore-5.0">C# Dependency Injection Framework</a>.

## What is DI?
[Link to Wikipedia](https://en.wikipedia.org/wiki/Dependency_injection)

> Dependency injection (DI) is a technique in which an object receives other objects that it depends on. These other objects are called dependencies. 
> In the typical "using" relationship the receiving object is called a client and the passed (that is, "injected") object is called a service. 
> The code that passes the service to the client can be many kinds of things and is called the injector. Instead of the client specifying which service 
> it will use, the injector tells the client what service to use. The "injection" refers to the passing of a dependency (a service) into the object 
> (a client) that would use it. 

## Highlights
- **Compile-time** - teloc uses the powerful rust type system check for the existence of dependencies that have the
proper lifetime at compile-time. This means you cannot compile your code if a required dependency has not been registered
or if it's lifetime is shorter to what's requested. If your code compiles, that means it runs!
- **Zero-overhead** - teloc uses only zero-overhead abstractions such as traits, generics, newtypes and unit types, and
compile-time resolving of dependencies, so you don't worry about overhead at runtime.
- **Simple API** - teloc provides you a simple API with only one struct and one attribute macro needed for working with
library.
- **Integration with existing enviroment** - teloc can be used with any existing frameworks like actix-web, warp, rocket. 
Now there is only support for actix-web as you can see [in the example](/examples/actix_example).

## How to use
There are one type can be provider of services: `ServiceProvider`. It used as store for dependencies with
`Instance` and `Singleton` lifetimes, and for declaring all dependencies using `.add_*()` methods. It can be forked to
create a local scope with local instances.

There are four lifetimes for dependencies:
1. `Transient`. Service will be created when resolves. Can depend on dependencies with anything lifetime.
2. `Singleton`. Service will be created once at `ServiceProvider` when it resolved (lazy). Can depend on dependencies 
with anything lifetime. Cannot depend on services from forked `ServiceProvider` instances.
3. `Instance`. Dependency was created outside of `ServiceProvider` and can be used by any other dependency.

How to work:
1. Declare your structs.
2. Create constructors and add `#[inject]` macro on its.
3. Create a `ServiceProvider` object.
4. Add your services and dependencies using `ServiceProvider::add_*` methods.
5. Fork `ServiceProvider` if you need to create local scope.
6. Get service from provider using `.resolve()` method.
7. Work with service.

Example:
```rust
use teloc::*;

// Declare your structs
struct ConstService {
    number: i32,
}
// #[inject] macro is indicate that dependency can be constructed using this function
#[inject]
impl ConstService {
    pub fn new(number: i32) -> Self {
        ConstService { number }
    }
}

// Derive macro can be used when all fields implement `Dependency` trait, but 
// we recommend using the #[inject] macro it in production code instead.
#[derive(Dependency)]
struct Controller {
    number_service: ConstService,
}

fn main() {
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
        .add_instance(10);
    // Get dependency from `ServiceProvider`
    let controller: Controller = scope.resolve();
    assert_eq!(controller.number_service.number, 10);
}
```

For documentation see [page on docs.rs](https://docs.rs/teloc/).

For more examples see [examples folder](/examples) or [tests folder](/teloc/tests).

## Comparison with other DI frameworks
<table>
<tr>
<td><b>Feature</b></td>
<td>teloc</td>
<td><a href="https://github.com/Mcat12/shaku">shaku</a></td>
<td><a href="https://github.com/dmitryb-dev/waiter">waiter_di</a></td>
</tr>
<tr>
<td>Compile-time checks</td>
<td>Yes</td>
<td>Yes</td>
<td>Yes</td>
</tr>
<tr>
<td>Can be used without dyn traits</td>
<td>Yes</td>
<td><a href="https://github.com/p0lunin/teloc/issues/8">Yes</a></td>
<td>Yes</td>
</tr>
<tr>
<td>Many service providers in one app</td>
<td>Yes</td>
<td>Yes</td>
<td>No</td>
</tr>
<tr>
<td>Different lifetimes</td>
<td>Yes</td>
<td>Yes</td>
<td>No</td>
</tr>
</table>

## How to read errors
Sometimes `teloc` can give strange large errors. But no panic! You can define your problem by read the <a href="HOW-TO-READ-ERRORS.md">manual</a> of reading errors.

## Pro tips
This section contains pro tips that you might want to use when working with the library.

### Get type of instance of `ServiceProvider`
It will be useful when you want to store an instance of `ServiceProvider` in a struct or return from a function or 
pass as an argument.

What you need:
1. Paste following code after `ServiceProvider` initialization: `let () = service_provider;`.
2. Compiler will give you very big terrible type starting with `teloc::ServiceProvider<...>`.
3. Copy that type into type alias, for example `type ConcreteSP = /*compiler output*/;`.
4. Use `ConcreteSP` when you want write `ServiceProvider` instance type.
5. If you change `ServiceProvider` initialization repeat steps 1-4.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
