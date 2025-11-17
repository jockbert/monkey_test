
# Changelog

[Show diff of unreleased changes on GitHub](https://github.com/jockbert/monkey_test/compare/v0.8.1...main).

## Release 0.8.0 (2025-11-17) [diff](https://github.com/jockbert/monkey_test/compare/v0.7.5...v0.8.0)

This release alleviates a problem with panicing panic-handling code.

## Release 0.8.0 (2025-11-13) [diff](https://github.com/jockbert/monkey_test/compare/v0.7.5...v0.8.0)

This release renames modules `monkey_test::gen` and `monkey_test::shrink` to `monkey_test::gens` and `monkey_test::shrinks`.
Since these modules are collections of genereators and shrinkers, the
plural 's' is a good fit and does not change the API nor nomenclature too much.

The Monkey Test library it self is still kept at Rust edition
2021, just to be able to stay at the current MSRV 1.73 and be usable
to as many users as possible.

By renaming module to `monkey_test::gens`, 2024 Rust edition users of library
do not need to refer to the module with 'r#gen' but instead
just `gens`. For more details, see
<https://doc.rust-lang.org/edition-guide/rust-2024/gen-keyword.html>

### New features

* Improved support for Rust edition 2024, by not using the in 2024 edition
  reserved word `gen` as module name `monkey_test::gen` in Monkey Test.
  The module is renamed from `monkey_test::gen` to `monkey_test::gens`.

### Other changes

* Rename module `monkey_test::shrink` to `monkey_test::shrinks`.
  For consitency, renaming to having plural suffix 's', just as
  the rename of module `monkey_test::gen` to `monkey_test::gens`.

* Updating dependencies `rand` and `rand_chacha` to the latest versions 0.9
  and 0.9.2 respectively.

* Reimplement built in generators for type `isize`.
  Built in support for type `isize` is removed in underlying lib `rand`, so
  replaing with other implementation piggy-backing in generators for type `i64`.
  For details on lost support in `rand`, see <https://rust-random.github.io/book/update-0.9.html>

## Release 0.7.5 (2025-11-05) [diff](https://github.com/jockbert/monkey_test/compare/v0.7.4...v0.7.5)

This release adds MSRV (Minimum Supported Rust Version), which should give some
guidance to users of this library.

## Release 0.7.4 (2025-10-31) [diff](https://github.com/jockbert/monkey_test/compare/v0.7.3...v0.7.4)

This release makes the crate buildable again by pinning down version on dependency.

### Other changes

* Pin down dependency `rand_chacha` to version 0.3.1 instead of the floating
  version 0. Version 0.9.0 of `rand_chacha` introduces API changes that
  currently breaks `monkey_test`.
* Only expose some functions in test-configuration, in order to remove
  dead code warnings.
* Eliminate some clippy warnings.

## Release 0.7.3 (2024-06-06) [diff](https://github.com/jockbert/monkey_test/compare/v0.7.2...v0.7.3)

This release focuses on adding filtering to generators and shrinkers.

### New features

* Now, there are filtering in generators and shrinkers. Let say that, you want
  to generate any `u8` value besides zero. This can now be achieved by using
  generator `monkey_test::gen::u8::any()` and just filter out the value zero.
  The filtering is also propagated to the associated shinker too, so even if
  failure is found, the value zero will not be one of the shrinked examples used
  when searching for a smaller failure.
* If example filtering is too heavy in either generator or shrinker, the
  filtering will complain about it by throwing a panic with a hopefully
  informative message.
* Catch panic and treat it as a property failures in all monkey_test assert
  methods.

  A property can panic for some example even tough for instance asserting
  equality. These panics should also be caught and treated as a property
  failure.

  Previously, unexpectedly panicking in non-panic-related test, monkey_test
  did not even say for which example it occurred, basicly just aborting
  the normal monkey_test procedure. This is now fixed and panics are
  caught and treated as any other failure in the monkey_test procedure.

## Release 0.7.2 (2024-05-09) [diff](https://github.com/jockbert/monkey_test/compare/v0.7.1...v0.7.2)

This release focuses on adding valid-range-aware integer shrinkers.

### New features

* Improve default integer shrinker, so it keeps track of the range of valid
  values and only provide shrunk candidates within that range. This is useful
  if integer generator is given an explicit range for the examples generated,
  then this range is also used in the shrinker, so in case of a failure also
  the shrinked candidates are kept within this given range.

### Other changes

* Improve the main documentation with additional examples in the common types
  of property-based tests and add a table of contents.

## Release 0.7.1 (2024-04-13) [diff](https://github.com/jockbert/monkey_test/compare/v0.7.0...v0.7.1)

Minor release only updating documentation and adding missing release details in
this changelog.

## Release 0.7.0 (2024-04-13) [diff](https://github.com/jockbert/monkey_test/compare/v0.6.0...v0.7.0)

This release focuses on adding alternative monkey test assert variants and more
informative panic messages when a property fails.

### New features

* Add possibility to add property title. Having this extra text label can be
  handy when having several small properties in the same unit test.

* Add `zip_n` for up to 6 parts, as an extension to all generators.

* Add failure reason text to `MonkeyResult::MonkeyErr`. This opens up for other
  types of property failure modes than just being true or false, like equality
  or panicing failure modes.

* Add `assert_no_panic`. It adds the possibility to verify that a property do
  not panic.

* Add `assert_eq`. It adds the possibility to compare two values against
  each other. In contrast to `assert_true`, `assert_eq`
  gives feedback on what the expected and actual values
  are for a failed property.

* Add `assert_ne`. Method `assert_ne` is added for symmetry compared to `assert_eq`.

* Include other failures found while shrinking in property panic message.
  Can be useful to see other failures too, and not only the minimal
  one, in property failure message.

### Breaking changes

* Adding public struct field `MonkeyResult::MonkeyErr.reason`.
* Adding public struct field `MonkeyResult::MonkeyErr.title`.
* Adding public struct field `ConfAndGen.title`.

### Other changes

* Preparing name change by deprecating `gen::bool::evenly()` and adding
  `gen::bool::any()` as a replacement.

* Preparing name change by deprecating `test_property` and adding `test_true`
  as a replacement.

### Bugfixes

* Use different seeds for the different parts of generated tuple in `gen::zip`.
  Earlier, same seed was used for the different parts of tuple
  generator (`zip`). This lead to tuples where both parts/items
  of tuple got the same value, if same generator type happened to be used
  for both tuple parts/items, only generating symmetric tuples
  like `(42, 42)` and `(1337, 1337)`.

## Release 0.6.0 (2024-03-28) [diff](https://github.com/jockbert/monkey_test/compare/v0.5.0...v0.6.0)

Release with focus on adding float generators and float shrinkers.

### New features

* Enable generators created from closure. See function `gen::from_fn`.
* Enable shrinkers created from closure. See function `shrink::from_fn`.
* The annotation `#[track_caller]` is added to function
  `monkey_test().assert_true()`, in order to a better location indication in
  panics emitted on property failure.
* Add generators for float values. See modules `gen::f32` and `gen::f64`.
* Add shrinkers for float values. See function `shrink::float`.

### Breaking changes

* Arguments given to `monkey_test::gen::bool::with_ratio` are changed from
  type u32 to u8. The change can of course create compile problems but will in
  practice likely have low impact, for two reasons. Firstly,
  the random generator seem to not be fine grained enough
  that the loss of precision matters in practice. Secondly, having
  ratios smaller than 1/256, will likely have a small impact on
  tested properties only using 100 examples by default.

* Integer generators ar enow using the more restrictive type constraint
  `num_traits::PrimInt`.

### Other changes

* Need for dependency `min_max_traits` is eliminated.
* Need for test dependency `num` is eliminated.

## Release 0.5.0 (2024-03-02) [diff](https://github.com/jockbert/monkey_test/compare/v0.4.0...v0.5.0)

Release with focus on improving vector generator and vector shrinker performance.

### New features

* Add generator for bool type. See module `gen::bool`.
* Add constant value generator. See generator function `gen::fixed::constant`.
* Add generator for progressive size of collections. See module `gen::sized`.
  The sized generator is also integrated into and used in the existing vector
  generator.
* Add element shrinking in vector shrinker. This was earlier lacking and
  shrinker only focused on rudimentary trying to reduce the vector size.
* Add aggressive/eager vector shrinking effort as first step in vector shrinker,
  in order to increase the overall effectiveness of the vector shrinker.
* Greatly improve integer shrinking speed.
  By first trying some candidates exponentially closer to target (zero),
  the integer shrinker has the potential for greatly reduced shrinking
  effort, compared to old way. In the old way, candidates was tried consecutive
  in decrementing order from  the original value, shrinking in linear time.
* Include some overweight to the value zero (0) in integer generators, if zero
  is included in the range generated. This should increase the possibility to
  find property bugs related to boundary cases related to the value zero.
  As before, some overweight is also given to the extremes (min and max) of
  the integer range generated.

### Breaking changes

* Rename shrinker `shrink::number()` to `shrinker::int()`, in order to make
  the shrinker name more specific and distinct. The same applies to module
  `shrink::num_shrink` which is renamed to `shrink::integer`.
  This is a preparation step for in the future also adding
  shrinkers for types `f32` and `f64`, which are also numbers, but are types
  that will not be supported by the existing integer shrinker, hence the name
  clarification.

### Bugfixes

* Avoid using same fixed seed (0) when testing properties.
* Improve shrinking performance, by renewing shrinking candidates when smaller
  failure is found.

  Earlier, when smaller failure example was found in shinking phase, the
  same original shrinker candidates iterator was further used, not
  taking advantage of the newly found smaller failure.

  Now, a new shrink candidate iterator is taken into use, using the new
  smaller failure as source. This should vastly improve the
  shrinking effectiveness, by using the newly found failure as a new smaller
  and improved (reduced) base case for all future candidates tested.

## Release 0.4.0 (2024-02-10) [diff](https://github.com/jockbert/monkey_test/compare/v0.3.0...v0.4.0)

Release with focus on functional style generator composition with `zip` and
`map`.

### New features

* Add possibility to zip generators together, with `gen::zip`, into generators of
  tuples.
* Add possibility to map generators from on type to another, with `gen::map`.
  Together with zipping, this can be used for creating generators and shrinkers
  for more complex types like structs.
* Explicitly include extremes in integer generators. Change behaviour
  of integer generators to having 2% extra occurrences of extreme
  values (minimum and maximum).

  The value of this is that you have a completely random generator of
  all possible u64-type values, the chance of actually testing the
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
  should make more sense when adding assert methods to `MonkeyResult`, like
  `MonkeyResult::assert_minimum_failure`.

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
  distribution or other distribution of users choosing.
* Add generators to mix together values from other generators, with either even
  distribution or other distribution of users choosing.
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
   generic arguments for shrinker on all places used.
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
