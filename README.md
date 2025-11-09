
# Monkey Test

[![monkey test logo](assets/monkeytest-banner-full-logo.png "Monkey Test logotype")](assets/monkeytest-logo.png)

A [property based testing](https://en.wikipedia.org/wiki/Software_testing#Property_testing)
*(PBT)* tool like
[QuickCheck](https://github.com/nick8325/quickcheck),
[ScalaCheck](https://scalacheck.org/) and
[similar libraries](https://en.wikipedia.org/wiki/QuickCheck), for
the Rust programming language.

> **☝️ Note!** This library is in active development.
> Parts of the functionality are missing and API can undergo changes.
> For details on recent changes, see the [CHANGELOG](CHANGELOG.md).

## Example

```rust
#[cfg(test)]
mod tests {
    use monkey_test::*;

    #[test]
    #[should_panic(expected = "Property failed!\nFailure: 15")]
    fn test_that_will_fail() {
        monkey_test()
            .with_generator(gens::u8::any())
            .assert_true(|x| x < 15);
    }
}
```

## Getting started

In `Cargo.toml`, add

```toml
[dev-dependencies]
monkey_test = "0"
```

Then try some small example, like the one above.

## Documentation and how-to guide

*Full documentation:* See file [DOCUMENTATION.md](./DOCUMENTATION.md),
also found as part of the
[source code documentation at docs.rs](https://docs.rs/monkey_test/).
It tries to be a complete how-to guide to using Monkey Test and also gives
a general introduction to property based testing.

*Additional examples:* See [tests/basic_usage.rs](tests/basic_usage.rs)
and other files in the [test folder](tests).

## Current status and missing parts

Currently, in versions 0.x.y, the library is in active development.
It is currently missing some parts, primarily built in generators
and shrinkers for:

* Strings.
* Commonly used data structures besides `Vec`.
* Recursive data structures.

Other known limitations:

* For now, float generators do not limit themselves to shrink to
  values within given generator range, but will by default shrink toward zero.
  For instance, let say that we create generator
  `monkey_test::gens::f64::ranged(10.0..100.0)`, the associated shrinker will not
  only try candidates within the given range `10.0..100.0`, but can also try other
  values like -10.0 and will ultimately try to shrink toward zero.

For details on recent changes, see the [CHANGELOG](CHANGELOG.md).

## Key design principles

The key design principles of the Monkey Test library are the following:

* *configurability and flexibility* - Leave a high degree of configurability
  and flexibility to the user by letting most details to be specified
  programatically. The aim is to have an declarative builder-style API like
  the Java library
  QuickTheories [(github)](https://github.com/quicktheories/QuickTheories).

* *powerful shrinking* - Good shrinkers is a really important aspect of a
  property based testing tool. Let say that the failing example is a vector
  of 1000 elements and only 3 of the elements in combination is the actual
  failure cause. You are then unlikely to find the 3-element combination,
  if the shrinking is not powerful enough.

* *composability for complex test examples* - Basic type generators and
  shrinkers are provided out of the box.
  User should also be able to generate and shrink more complex types, by
  composing together more primitive generators and shrinkers into more
  complex ones.
  The main inspiration here is the Scala library ScalaCheck
  [(homepage)](https://scalacheck.org/),
  which is phenomenal in this aspect, having the power to for example easily
  generate and shrink recursive data structures, by using composition.

* *minimize macro magic* - In order to keep the tool simple, just avoid macros
  if same developer experience can be provided using normal Rust code.
  Macro-use is a complex escape hatch only to be used when normal syntax
  is insufficient.

## Alternative libraries

There are other alternatives for property based testing in Rust.
The Monkey Test library exist for mostly subjective reasons, not liking the
API experience or the heavy use of macros and attributes in other libraries.
Your mileage may vary.

The most mature and widely adopted alternatives are
[Quickcheck](https://crates.io/crates/quickcheck) and
[Proptest](https://crates.io/crates/proptest). Currently, if you want to have a
production grade PBT library, choose one of these two. When in doubt, choose
Proptest, since it allows for custom generators and shrinkers.

Some other alternatives are [checkito](https://crates.io/crates/checkito) and
[diceprop](<https://crates.io/crates/diceprop>).

## License

Monkey test uses the [MIT license](LICENSE.txt).

## Contributions

This library needs feedback from users to become even better. Feel free to
[open a issue](https://github.com/jockbert/monkey_test/issues/new) or
[open a pull request](https://github.com/jockbert/monkey_test/compare).

All work in Monkey Test is licensed under the terms of the MIT license.
By submitting a contribution you are agreeing to license your work under those
terms.
