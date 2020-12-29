# teloc
<div>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
</div>
Teloc is simple, compile-time DI framework for Rust.

## What is DI?
[Link to Wikipedia](https://en.wikipedia.org/wiki/Dependency_injection)

> Dependency injection (DI) is a technique in which an object receives other objects that it depends on. These other objects are called dependencies. 
> In the typical "using" relationship the receiving object is called a client and the passed (that is, "injected") object is called a service. 
> The code that passes the service to the client can be many kinds of things and is called the injector. Instead of the client specifying which service 
> it will use, the injector tells the client what service to use. The "injection" refers to the passing of a dependency (a service) into the object 
> (a client) that would use it. 

## Highlights
- **Compile-time** - teloc uses powerful rust type system for lifetime and existing of dependencies checking in 
compile-time. That means that you cannot build your code if you do not register dependencies for all of registered 
dependencies or lifetime of dependency does not correspondence to requester. If your code is compile, that mean it run!
- **Zero-overhead** - teloc uses only zero-overhead abstractions such as traits, generics, newtypes and unit types, and
compile-time resolving of dependencies, so you don't worry about overhead in runtime.
- **Simple API** - teloc provides you a simple API with only one struct and one attribute macro needed for working with
library.

## How to use
There are 2 types can be provider of services: `ServiceProvider` and `Scope`. First used as store for dependencies with
`Instance` and `Singleton` lifetimes, and for declaring all dependencies using `.add_*()` methods. `Scope` can be 
created from `ServiceProvider` object by calling method `ServiceProvider::scope`.

There are four lifetimes for dependencies:
1. `Transient`. Service will be created when resolves. Can depend on dependencies with anything lifetime.
2. `Singleton`. Service will be created once at `ServiceProvider` when it resolved (lazy). Can depend on dependencies 
with anything lifetime.
3. `Instance`. Dependency was created outside of `ServiceProvider` and can be used by any other dependency.

Process of working with library:
1. Define your structs.
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
    let scope = sp.fork().add_instance(10);
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
<td>**Feature**</td>
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
<td>No</td>
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
3. Copy that type into type alias, for example `type ConcreteSP = /*copiler output*/;`.
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
