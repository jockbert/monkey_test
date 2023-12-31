# Monkey Test

Monkey Test is a
[property based testing (*PBT*)](https://en.wikipedia.org/wiki/poftware_testing#Property_testing)
tool like QuickCheck
[(Wikipedia)](https://en.wikipedia.org/wiki/QuickCheck)
[(github)](https://github.com/nick8325/quickcheck) and other deriviatives thereof.

## Property based testing core concepts

PBT is a complement to normal unit testing.

A normal unit test uses a singel specific example input and verifies it
against a specific outcome.

```rust
// Unit tests with hard coded examples with expected results
assert_eq!(1_f64.sqrt(), 1_f64);
assert_eq!(4_f64.sqrt(), 2_f64);
assert_eq!(9_f64.sqrt(), 3_f64);
assert_eq!(16_f64.sqrt(), 4_f64);
```

With PBT a property of your code is validated against an arbitrary number of
generated examples.
A propery is saying something more general about your code than a specific
example and outcome.
You often loose some specificity but can say something more general about
the code under test.
Further, using random examples in test can find aspects you missed when
manually choosing examples to test.

```rust
// Testing more general properties
use monkey_test::*;

monkey_test()
    .with_generator(gen::u64::ranged(2..))
    .assert_true(|n| (n as f64).sqrt() > 1.0)
    .assert_true(|n| (n as f64).sqrt() < n as f64);
```

So, what is the point of having the loose boundaries in the `sqrt`-properties
tested above? Is there any value in testing these properties?

The answer is that usually when code goes wrong or has a bug, the
return value is not just a little bit off, but many times it is way off and
fail spectacularly, like returning a negative value or panicing.

```rust
// Combining unit tests with more general properties
use monkey_test::*;

assert_eq!(1_f64.sqrt(), 1_f64);
assert_eq!(4_f64.sqrt(), 2_f64);
assert_eq!(9_f64.sqrt(), 3_f64);
assert_eq!(16_f64.sqrt(), 4_f64);

monkey_test()
    .with_generator(gen::u64::ranged(2..))
    .assert_true(|n| (n as f64).sqrt() > 1.0)
    .assert_true(|n| (n as f64).sqrt() < n as f64);
```

In short, combining general property based tests with some specific
unit tests is a powerful testing technique to both specify the precise behaviour
and finding bugs you did not forsee yourself.

## Nomenclature

- *Generator* - A source of random examples
- *Property* - Your parameterized test
- *Shrinker* - Generate smaller examples based on failing example, in order to
  simplify the failure

## Some common classes of properties to use

How do you write a useful property that is valid for all generated examples?
One baby step is to try parameterize an already existing example based test.
As an inspiration, here follow some common classes of properties to test.

### No explosion

Just shoot examples at code under test and make sure there are no errors and
no panics.

### Simplification

### Symmetry

Apply a function and its inverse and make sure you get back the same initial
value. Some examples of this is write and read back something, like save to
file system and load it again.
Another example is to generate some ground truth data, transform it to an
input format and make sure your business logic calculate an answer that
corresponds to the ground truth data.

```rust
// Testing the serialize and deserialize round trip for a parser
use monkey_test::*;

monkey_test()
   .with_generator(gen::u32::any())
   .assert_true(|ground_truth| {
      let input = ground_truth.to_string();
      let actual_output = input.parse::<u32>().unwrap();
      actual_output == ground_truth
   });
```

### Idempotens

Applying the same function many times generate the same result.

```rust
use monkey_test::*;

monkey_test()
   .with_generator(gen::i8::ranged(-127..))
   .assert_true(|n| n.abs() == n.abs().abs().abs().abs());
```

### Oracle

Compare the function result against other trusted means to get
the same result. Perhaps compare output to a model of one aspect,
analogous function, other existing implementation, unoptimized code or
legacy code. Perhaps compare the output of old and new code to enable
reckless refactoring.

```rust
// Example of analogous function to get to the same result
use monkey_test::*;

monkey_test()
   .with_generator(gen::u8::ranged(..128))
   .assert_true(|n| {
      let oracle_method = n + n;
      let tested_method = n * 2;
      tested_method == oracle_method
   });
```

### Induction

Show that some property holds for `P(0)` and that `P(n) + C = P(n+1)`.

```rust
// Via induction show that Vec::len() works as expected
use monkey_test::*;

// Induction base case with empty vector
assert_eq!{Vec::<i64>::new().len(), 0};

// Induction general case
monkey_test()
   .with_generator(gen::vec::any(gen::u8::any()))
   .assert_true(|original| {
      let mut modified = original.clone();
      modified.push(42);
      modified.len() == original.len() +1
   });
```

### Stateful testing

As one example, execute a series of commands against a stateful system, to
then verify some property of the system.

## Features

### Integer generators and shrinkers

Generators for all integer types `i8`, `i16`, `i32`, `i64`, `i128`, `isize`,
`u8`, `u16` `u32`, `u64` , `u128` and `usize`.

```rust
let bytes = monkey_test::gen::u8::any();
let some_longs = monkey_test::gen::i64::ranged(10..=20);
```

## Key design principles of the Monkey Test tool

- *configurability and flexibility* - Leave a high degree of configurability
   and flexibility to the user by letting most details to be specified
   programatically. The aim is to have an declarative builder-style API like
   the Java library
   QuickTheories [(github)](https://github.com/quicktheories/QuickTheories).

- *powerful shinking* - Good shrinkers is a really important aspect of a
   property based testing tool. Let say that the failing example is a vector
   of 1000 elements and only 3 of the elements in combination is the actual
   failure cause. You are then unlikely to find the 3-element combination,
   if the shrinking is not powerful enough.

- *composability for complex test examples* - Basic type generators and
   shrinkers are provided out of the box.
   User should also be able to genereate and shrink more complex types, by
   composing together more primitive generators and shrinkers into more
   complex ones.
   The main inspiration here is the Scala library ScalaCheck
   [(homepage)](https://scalacheck.org/),
   which is fenomenal in this aspect, having the power to for example easily
   generate and shrink recursive data structures, by using composition.

- *minimize macro magic* - In order to keep the tool simple, just avoid macros
   if same developer experience can be provided using normal Rust code.
   Macros-use is an complex escape hatch only to be used when normal syntax
   is insufficient.
