# teloc
<div>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
</div>
Teloc is compile-time, blazing fast DI framework for Rust.

## What is DI?
[Link to Wikipedia](https://en.wikipedia.org/wiki/Dependency_injection)

> Dependency injection (DI) is a technique in which an object receives other objects that it depends on. These other objects are called dependencies. 
> In the typical "using" relationship the receiving object is called a client and the passed (that is, "injected") object is called a service. 
> The code that passes the service to the client can be many kinds of things and is called the injector. Instead of the client specifying which service 
> it will use, the injector tells the client what service to use. The "injection" refers to the passing of a dependency (a service) into the object 
> (a client) that would use it. 

## Highlights
- **Compile-time** - teloc uses all power of functional paradigm of [frunk](https://github.com/lloydmeta/frunk) library, resolves all dependencies in compile-time. 
That means that you cannot build your code if you do not register dependencies for all of registered dependencies. If your code is compile, that mean it run!
- **Blazing fast** - teloc uses only zero-cost abstractions such as traits, generics, newtypes and unit types, so you don't worry about speed of resolving dependencies.
- **Simple for simple usage, hard for hard things** - teloc provides you a simple API for simple things when you wish to use teloc for trivial cases. But you can
customize process of resolving dependencies, create your own containers for dependencies and extend basic `ServiceProvider` your own methods!

## Example of usage
TODO

## Comparison with other DI frameworks
<table>
<tr>
<td>Library</td>
<td>Compile-time</td>
<td>Can be used without dyn traits</td>
<td>Many containers in one app</td>
</tr>
<tr>
<td>teloc</td>
<td>Yes</td>
<td>Yes</td>
<td>Yes</td>
</tr>
<tr>
<td><a href="https://github.com/Mcat12/shaku">shaku</a></td>
<td>Yes</td>
<td>No</td>
<td>Yes</td>
</tr>
<tr>
<td><a href="https://github.com/dmitryb-dev/waiter">waiter_di</a></td>
<td>Yes</td>
<td>Yes</td>
<td>No</td>
</tr>
</table>

## How to read errors
Sometimes `teloc` can give strange large errors. But no panic! You can define your problem by read the <a href="HOW-TO-READ-ERRORS">manual</a> of reading errors.

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
