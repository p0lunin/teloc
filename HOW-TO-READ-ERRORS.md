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
    - If it registered using `Scoped` lifetime (`add_scoped` method), than it can be resolved by clone in cases when
    `Service` implement `DependencyClone`, so if you want to resolve it by clone, check that it implement `DependencyClone`.
    Otherwise it can be resolved only by reference.
    Also check that your `provider` has the type `Scope`, not `ServiceProvider`.
    - If it registered using `Singleton` lifetime (`add_singleton` method), than it can be resolved by clone in cases when
    `Service` implement `DependencyClone`, so if you want to resolve it by clone, check that it implement `DependencyClone`.
    Otherwise it can be resolved only by reference.
