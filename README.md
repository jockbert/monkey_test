
# Monkey Test

![monkey test logo](assets/monkeytest-banner.png)

A [property based testing](https://en.wikipedia.org/wiki/Software_testing#Property_testing)
*(PBT)* tool like
[QuickCheck](https://github.com/nick8325/quickcheck),
[ScalaCheck](https://scalacheck.org/) and
[similar libraries](https://en.wikipedia.org/wiki/QuickCheck), for
the Rust programming language.

<mark>☝️ Note! This library is in active development.
Parts of functionality is missing and API can undergo changes.
For details on recent changes, see the [CHANGELOG](CHANGELOG.md).</mark>

## Example

```rust
#[cfg(test)]
mod tests {
    use monkey_test::*;

    #[test]
    #[should_panic(expected = "Property failed!\nFailure: 15")]
    fn test_that_will_fail() {
        monkey_test()
            .with_generator(gen::u8::any())
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

[The Monkey Test DOCUMENTATION](./DOCUMENTATION.md)
(also found on [docs.rs](https://docs.rs/monkey_test/))
shows how to use the library and tries to be a complete how-to guide to using
Monkey Test and property based testing in general.
Additional usage examples can be found in the source file
[tests/basic_usage.rs](tests/basic_usage.rs) and other files in
[test folder](tests).

## Current status and missing parts

Currently, in versions 0.x.y, the library is in active development.
It is currently missing some parts, primarly built in generators
and shrinkers for:

* Strings.
* Commonly used data structures besides `Vec`.
* Recursive data structures.

Other known limitations:

* For now, integer and float generators do not limit them self to shrink to
  values within given generator range, but will by default shrink toward zero.
  For instance, let say that we create generator
  `monkey_test::gen::i64::ranged(10..100)`, the associated shrinker will not
  only try candidates withing the given range `10..100`, but can also try other
  values like -10 and will ultimately try to shrink toward zero.

For details on recent changes, see the [CHANGELOG](CHANGELOG.md).

## Alternative libraries

There are other alternatives for property based testing in Rust.
The Monkey Test library exist for mostly subjective reasons, not liking the
API experience or the heavy use of macros and attributes in other libraries.
Your milage may vary.

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
By submitting a contribution you are agreeing to licence your work under those
terms.
