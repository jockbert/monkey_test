
[pre-alpha]: https://en.wikipedia.org/wiki/Software_release_life_cycle#Pre-alpha

# Monkey Test

![monkey test logo](assets/monkeytest-banner.png)

A [property based testing](https://en.wikipedia.org/wiki/Software_testing#Property_testing)
*(PBT)* tool like
[QuickCheck](https://github.com/nick8325/quickcheck),
[ScalaCheck](https://scalacheck.org/) and
[other deriviatives thereof](https://en.wikipedia.org/wiki/QuickCheck), for
the Rust programming language.

<mark>☝️ Warning! This library is in [pre-alpha] state.
Large parts of functionality is missing and API will undergo a lot of
change. For details on recent changes, see the [CHANGELOG](CHANGELOG.md).</mark>

## Example

```rust
#[cfg(test)]
mod tests {
    use monkey_test::*;

    #[test]
    #[should_panic(expected = "Property failed!\nCounterexample: 15")]
    fn test_that_will_fail() {
        monkey_test()
            .with_generator(gen::u8::any())
            .assert_true(|x| x < 15)
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

The Monkey Test [documentation](./DOCUMENTATION.md)
(also found on [docs.rs](https://docs.rs/monkey_test/))
shows how to use the library and tries to be a complete how-to guide to using
Monkey Test and property based testing in general.
Additional usage examples can be found in the source file
[tests/basic_usage.rs](tests/basic_usage.rs) and other files in test folder.

## Current status and missing parts

Currently, in versions 0.x.y, the library is not ready for production use.
It is among other things missing some vital parts, primarly built in generators
and shrinkers for:

* Floating point numbers `f32` and `f64`.
* Strings.
* Commonly used data structures besides `Vec`.
* Recursive data structures.

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
