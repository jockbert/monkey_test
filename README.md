
[pre-alpha]: https://en.wikipedia.org/wiki/Software_release_life_cycle#Pre-alpha

# Monkey Test

![monkey test logo](assets/monkeytest-banner.png)

A [property based testing](https://en.wikipedia.org/wiki/Software_testing#Property_testing)
*(PBT)* tool like
[QuickCheck](https://github.com/nick8325/quickcheck),
[ScalaCheck](https://scalacheck.org/) and
[other deriviatives thereof](https://en.wikipedia.org/wiki/QuickCheck), for
the Rust programming language.

<mark>❗This library is in [pre-alpha] state.
Large parts of functionality is missing and API will undergo a lot of
change.</mark>

## Example

```rust
use monkey_test::*;

#[test]
#[should_panic(expected = "Property failed!\nCounterexample: 15")]
fn test_that_will_faill() {
    monkey_test()
        .with_generator(gen::u8::any())
        .assert_true(|x: u8| x < 15)
}
```

## Getting started

In `Cargo.toml`, add

```toml
[dev-dependencies]
monkey_test = "0"
```

Then try some small example, like the one above.

## Documentation

The Monkey Test [documentation (docs.rs)](https://docs.rs/monkey_test/) contains
additional usage examples and tries to be a complete guide to using Monkey Test
and property based testing in general.

## License

Monkey test uses the [MIT license](LICENSE.txt).
