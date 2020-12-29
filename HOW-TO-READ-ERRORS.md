# How to read errors
In this file is described common errors and how it repair.

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
It means that provider cannot resolve that `SERVICE`. Possible reasons are:
1. You do not register this dependency in `ServiceProvider`.
2. If you register the `Service`, than check by which lifetime:
    - If it registered using `Transient` lifetime (`add_transient` method), than it can be resolved only by ownership,
    so check that it not resolved by reference.
    - If it registered using `Singleton` lifetime (`add_singleton` method), than it can be resolved by clone in cases when
    `Service` implement `DependencyClone`, so if you want to resolve it by clone, check that it implement `DependencyClone`.
    It implement by default for `Rc`, `Arc` and `&T`. Otherwise it can be resolved only by reference.
3. Check that you register all dependencies for the `SERVICE` by steps 1-2.
