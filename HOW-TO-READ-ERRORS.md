# How to read errors
This file describes common errors and how to fix them.

## `The trait bound ...` when resolve dependency by hand
Error looks like this:
```
error[E0277]: the trait bound ...  
--> folder\file.rs:24:44
   |
24 |     let service: SERVICE = provider.resolve();
   |                                     ^^^^^^^ the trait `teloc::Resolver<'_, _, SERVICE, ...   
   |
   ...
```
It means that the provider cannot resolve that `SERVICE`. Possible reasons are:
1. You do not register this dependency in `ServiceProvider`.
2. If you register the `Service`, then check by which lifetime:
    - If it is registered using `Transient` lifetime (`add_transient` method), then it can be resolved only by ownership,
    so check that it is not resolved by reference.
    - If it is registered using `Singleton` lifetime (`add_singleton` method), then it can be resolved by clone in cases when
    `Service` implement `DependencyClone`, so if you want to resolve it by clone, check that it implements `DependencyClone`.
    It is implemented by default for `Rc`, `Arc` and `&T`. Otherwise it can be resolved only by reference.
3. Check that you register all dependencies for the `SERVICE` by steps 1-2.
