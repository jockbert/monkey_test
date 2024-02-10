
# Changelog

[Show diff of unreleased changes on GitHub](https://github.com/jockbert/monkey_test/compare/v0.4.0...main).

## Release 0.4.0 (2024-02-10) [diff](https://github.com/jockbert/monkey_test/compare/v0.3.0...v0.4.0)

Release with focus on functional style generator composition with `zip` and
`map`.

### New features

* Add possibility to zip genrators together, with `gen::zip`, into generators of
  tuples.
* Add possibility to map generators from on type to another, with `gen::map`.
  Together with zipping, this can be used for creatign generators and shrinkers
  for more complex types like structs.
* Explicitly include extremes in integer generators. Change behaviour
  of integer generators to having 2% extra occurrences of extreme
  values (minimum and maximum).

  The value of this is that you have a completely random generator of
  all possible u64-type values, the chance of actualy testing the
  extreme values (in this case 0 and 2^64-1) is extremely small in the
  100 examples actually applied to a propery. Testing with the extreme
  values are important, if you want to find bugs.
* Add test-generator `gen::fixed::in_loop`.
* Make sure mix-generators reuse shrinker from first of given generators, so
  there is at least on shrinker used in mix-generators by default.
* Add method `MonkeyResult::assert_minimum_failure`, which should be a useful
  diagnostics and demo tool when evaluating shrinkers, generators and
  properties. The method asserts that the minimum failure is the expected one.

### Breaking changes

* Rename `ConfAndGen::check_true` to `ConfAndGen::test_property`. This naming
  should make more sense when adding assert methods to `MonkeyResult`, like `MonkeyResult::assert_minimum_failure`.

## Release 0.3.0 (2024-01-19) [diff](https://github.com/jockbert/monkey_test/compare/v0.2.0...v0.3.0)

The big (breaking) change in this version is the change to using boxed
`Gen` and `Shrink` by default everywhere in the Monkey Test API.

This enables simplifying the Monkey Test API by reducing the need for
extensive use of generics, especially in nested decorator-style structs
having other generators and shrinkers within.

Also, making `Gen` and `Shrink` into true trait objects, enable using
arbitrary collections of generators and shrinkers in the API. This
come in handy when also adding the possbility to mix generators and
shrinkers respectively.

Further, the number of exposed concrete implementations (structs) of
the generator and shrinker traits, can be reduced in the public API.
It should simplify the details for library user, making it more user
friendly. Potential performance loss of always using `Box` everyhere is
not a problem until shown otherwise.

### New features

* Expose Box type aliases `BoxGen<E>` and `BoxShrink<E>` for `Gen<E>` and
  `Shrink<E>` in API.
* Add generators to pick values among given set of values, with either even
  distribution or oter distribution of users choosing.
* Add generators to mix together values from other generators, with either even
  distribution or oter distribution of users choosing.
* Add success count to monkey test assert message.

### Breaking changes

* Rename `SomeIter` to `BoxIter`, so it aligns with BoxGen and BoxShrink.

## Release 0.2.0 (2023-11-10) [diff](https://github.com/jockbert/monkey_test/compare/v0.1.1...v0.2.0)

### New features

* Enable chaining generators together, using `Gen.chain`.
* Add generators for all integer types for type, from `u8` to `i128` and
  `usize` and `isize`.
* Enable monkey test to do asserts in sequence. Now the monkey test
  configuration setup can be reused for multiple calls to `assert_true` in a
  single call chain.

    ```rust
    monkey_test()
        .with_generator(gen::u64::ranged(2..))
        .assert_true(|n| (n as f64).sqrt() > 1.0)
        .assert_true(|n| (n as f64).sqrt() < n as f64);
        .assert_true(|n| n == n);
    ```

### Breaking changes

* Change generator example and shrinker template parameters `E` and `S` in
   `Gen<E, S>` to a Generic Associated Types (GAT) named `Gen::Example` and
   `Gen::Shrink`, in order to try reducing the "template hell" when
   constructing and using generators.
* Configuration structs are (moved and) renamed from `Monkey` and
   `MonkeyWithGen` to `Conf` and `ConfWithGen`.
* Type alias `SomeShrink<E> = Box<dyn Shrink<E>>` is replaced with use of
   generic arguments for shrinker on all places ursed.
* Rename `Gen.iter` to `Gen.examples`. This hopefully makes the purpose of
   the returned iterator more clear.
* Include shinker in generator, adding method `Gen.shrinker`. This enables
   distributing a default shrinker with given generator, reducing the need to
   explicitly configure a shrinker to use, when applying the monkey test tool
   in a test.
* Rename `NumDecrementShrink` to `NumShrink` (and remove unused types
   `NumericShrinks` and `NumericShrink`).
* Rename `SliceGen` to `SequenceGen` to hopefully convey the purpose more
   clearly.

### Other changes

* Extract main documentation to separate file.

## Release 0.1.1 (2023-09-26) [diff](https://github.com/jockbert/monkey_test/compare/v0.1.0-proof-of-concept...v0.1.1)

Mostly contains improvements to the documentation.

## Release 0.1.0 (2023-09-22) [diff](https://github.com/jockbert/monkey_test/compare/init...v0.1.0-proof-of-concept)

Add initial embryo for the monkey test library.