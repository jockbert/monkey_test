**<span style="color:#EF643F">WARNING! This library is in [pre-alpha] state.
Large parts of functionality is missing and API will undergo a lot of change.</span>**

[pre-alpha]: https://en.wikipedia.org/wiki/Software_release_life_cycle#Pre-alpha

# Monkey Test

![monkey test logo](assets/monkeytest-banner.png)

A [property based testing](https://en.wikipedia.org/wiki/Software_testing#Property_testing) *(PBT)* tool like [QuickCheck](https://github.com/nick8325/quickcheck), [ScalaCheck](https://scalacheck.org/) and [other deriviatives thereof](https://en.wikipedia.org/wiki/QuickCheck), for the Rust programming language.

## Example

```rust

#[test]
#[should_panic(expected = "Property failed!\nCounterexample: 15")]
fn test_that_will_faill() {
    monkey_test()
        .with_generator(gen::u8::any())
        .assert_true(|x: u8| x < 15)
}
```

## Documentation

For a deper introduction of property based testing and how to use this library, see the rustdoc documentation.

## License

Monkey test uses the [MIT license](LICENSE.txt).
