# Monkey Test

Monkey Test is a
[property based testing (*PBT*)](https://en.wikipedia.org/wiki/software_testing#Property_testing)
tool like QuickCheck
[(Wikipedia)](https://en.wikipedia.org/wiki/QuickCheck)
[(github)](https://github.com/nick8325/quickcheck) and similar libraries.

## Contents

* [Property based testing core concepts](#property-based-testing-core-concepts)
* [Nomenclature](#nomenclature)
* [Features](#features)
  * [Generators and shrinkers for basic types](#generators-and-shrinkers-for-basic-types)
  * [Generators and shrinkers for collections](#generators-and-shrinkers-for-collections)
  * [Pick values and mix generators](#pick-values-and-mix-generators)
  * [Compose generators and shrinkers for more complex types](#compose-generators-and-shrinkers-for-more-complex-types)
  * [Create generators and shrinkers from scratch](#create-generators-and-shrinkers-from-scratch)
* [How to write a property](#how-to-write-a-property)
  * [No explosion](#no-explosion)
  * [Simplification](#simplification)
  * [Symmetry](#symmetry)
  * [Idempotence](#idempotence)
  * [Invariance](#invariance)
  * [Oracle](#oracle)
  * [Induction](#induction)
  * [Stateful testing](#stateful-testing)

## Property based testing core concepts

PBT is a complement to normal unit testing.

A normal unit test uses a single specific example input and verifies it
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
A property can loose some specificity, but can usually say something more general
about the code under test compared to a specific test example and outcome.
Further, using random examples in test can find aspects you missed when
manually choosing examples to test.

```rust
// Testing more general properties
use monkey_test::*;

monkey_test()
    .with_generator(gens::f64::ranged(2.0..))
    .title("Lower bound")
    .assert_true(|n| n.sqrt() > 1.0)
    .title("Upper bound")
    .assert_true(|n| n.sqrt() < n);
```

So, what is the point of having the loose boundaries in the `sqrt`-properties
tested above? Is there any value in testing these properties?

The answer is that usually when code goes wrong or has a bug, the
return value is not just a little bit off, but many times it is way off and
fail spectacularly, like returning a negative value or panicing.

In short, combining general property based tests with some specific
unit tests is a powerful testing technique to both specify the precise behaviour
and finding bugs you did not foresee yourself.

## Nomenclature

* *Generator* - A source of random examples
* *Property* - Your parameterized test
* *Shrinker* - Generate smaller examples based on failing example, in order to
  simplify the failure

## Features

For a complete guide of all features in the Monkey Test library, refer to the
[generated API documentation (docs.rs)](https://docs.rs/monkey_test).
Additional usage examples can be found in
[code repository test folder (github.com)](https://tests).
A summary is given below.

### Generators and shrinkers for basic types

Generators for `bool`, `f32`, `f64` and for all integer types
`i8`, `i16`, `i32`, `i64`, `i128`, `isize`,
`u8`, `u16` `u32`, `u64` , `u128` and `usize`.

```rust
use monkey_test::*;
let bytes = gens::u8::any();
let some_longs = gens::i64::ranged(10..=20);
let mostly_true = gens::bool::with_ratio(1,20);
```

There are some more specialized generators. In module
`gens::sized` there are generators that return progressively larger and larger
values, suitable for controlling the size of generated collections. In module
`gens::fixed` there are generators that do not use randomness, which can be
useful sometimes.

```rust
use monkey_test::*;
let progressively_larger_sizes: BoxGen<usize> = gens::sized::default();
let always_the_same_value: BoxGen<i32> = gens::fixed::constant(42);
```

### Generators and shrinkers for collections

There is a generator and a shrinker for vectors.

```rust
use monkey_test::*;
let int_vectors: BoxGen<Vec<i16>> = gens::vec::any(gens::i16::any());

monkey_test()
   .with_generator(int_vectors)
   .test_true(|vec| vec.iter().all(|&n| n <= 1337) )
   .assert_minimum_failure(vec![1338]);
```

### Pick values and mix generators

Create generators that pick among values and mix values from different
generators

```rust
use monkey_test::*;
let fruits = gens::pick_evenly(&["banana", "apple", "orange"]);
let nuts = gens::pick_evenly(&["peanut", "almond", "pecan"]);
let snacks = gens::mix_with_ratio(&[(3, nuts), (1, fruits)]);
```

### Compose generators and shrinkers for more complex types

Generators and shrinkers for more complex types can be constructed from more
basic ones, using one of `zip`, `zip_3`, ..., `zip_6` together with `map`
and `filter`.
When constructing generators this way, you automatically also get a shrinker for
the complex type.

```rust
use monkey_test::*;

#[derive(Clone)]
struct Point {x: u16, y: u16}

let points: BoxGen<Point> = gens::u16::any()
   .zip(gens::u16::any())
   .map(|(x, y)| Point{x, y}, |p| (p.x, p.y))
   .filter(|p| p.x != p.y);

#[derive(Clone)]
struct Color {r: u8, g: u8, b: u8, a: u8}

let colors: BoxGen<Color> = gens::u8::any()
   .zip_4(gens::u8::any(), gens::u8::any(), gens::u8::any())
   .map(|(r, g, b, a)| Color{r, g, b, a}, |c| (c.r, c.g, c.b, c.a))
   .filter(|c| c.r > 10);
```

### Create generators and shrinkers from scratch

For implementing a generator on your own, you only need to implement the
[Gen] trait.

```rust
use monkey_test::*;
// Use the randomization source of your choosing
use rand::Rng;
use rand::SeedableRng;

#[derive(Clone)]
struct DiceGen {
   /// Die side count
   side_count: u32,
}

impl Gen<u32> for DiceGen {
    fn examples(&self, seed: u64) -> BoxIter<u32> {
        let distr =
            rand::distributions::Uniform::new_inclusive(1, self.side_count);
        let iter =
            rand_chacha::ChaCha8Rng::seed_from_u64(seed).sample_iter(distr);
        Box::new(iter)
    }

    fn shrinker(&self) -> BoxShrink<u32> {
        shrink::int()
        // Use shrink::none() for not providing any shrinking.
    }
}

fn dice_throw_generator_from_struct(side_count: u32) -> BoxGen<u32> {
    Box::new(DiceGen { side_count })
}
```

Some boilerplate code can be eliminated and the same functionality can be
achieved by using [gens::from_fn] instead of implementing the [Gen] trait.

```rust
use monkey_test::*;
use rand::Rng;
use rand::SeedableRng;

fn dice_throw_generator_from_fn(side_count: u32) -> BoxGen<u32> {
    gens::from_fn(move |seed| {
        let distr = rand::distributions::Uniform::new_inclusive(1, side_count);
        rand_chacha::ChaCha8Rng::seed_from_u64(seed).sample_iter(distr)
    })
    .with_shrinker(shrink::int())
}
```

Similarly, a shrinker can be implemented by either implementing the [Shrink]
trait directly, or just make use of [shrink::from_fn].

## How to write a property

How do you write a useful property that is testable and valid for all generated
examples?
One baby-step is to try parameterize an already existing example based test.
As an inspiration, here follow some common classes of properties to test.

### No explosion

Just shoot examples at code under test and make sure there are no errors and
no panics.

```rust,should_panic
// Testing that there are no panics, but will panic on division by zero for
// example range 700..800.
use monkey_test::*;

monkey_test()
   .with_example_count(1_000)
   .with_generator(gens::i32::ranged(-10_000..10_000))
   .assert_no_panic(|n| { let _ = 1/(n / 100 - 7); });
```

```rust,should_panic
// Testing that there are no Err, but will fail on -1 and lower values.
use monkey_test::*;

monkey_test()
   .with_generator(gens::i8::any())
   .assert_true(|n| u8::try_from(n).is_ok() );
```

### Simplification

Since the examples used are random, i.e. unknown, it can be hard to be specific
about what result to expect. However, using the "loose boundaries" principle,
you can usually at least specify some relaxed model - a simplified property
to verify.

```rust
// We can at least specify that all results from `abs()` should be positive, if
// not being i8 minimum value which does not have a positive counterpart.
use monkey_test::*;

monkey_test()
   .with_generator(gens::i8::any())
   .assert_true(|n| n == i8::MIN || n.abs() >= 0 );
```

### Symmetry

Apply a function and its inverse and make sure you get back the same initial
value. Some examples of this is write and read back something, like save to
file system and load it again.
Another example is to generate some ground truth data, transform it to an
input format and make sure your business logic calculate an answer that
corresponds to the ground truth data.

```rust
// Testing the serialize and deserialize round trip for a parser.
use monkey_test::*;

monkey_test()
   .with_generator(gens::u32::any())
   .assert_eq(|n| n, |n| n.to_string().parse::<u32>().unwrap());
```

### Idempotence

Applying the same function many times generate the same result.
This can be useful for showing that there is no hidden state
affecting the result of the code under test.

```rust
// Absolute value stays the same through several applications of "abs".
use monkey_test::*;

monkey_test()
   .with_generator(gens::i8::ranged(-127..))
   .assert_eq(|n| n.abs(), |n| n.abs().abs().abs().abs());
```

### Invariance

A property or value that is not changed by some specific operation or
transformation.

```rust
// The negation operation does not affect the absolute value.
use monkey_test::*;

monkey_test()
   .with_generator(gens::i8::ranged(-127..))
   .assert_eq(|n| n.abs(), |n| (-n).abs());
```

### Oracle

Compare the function result against other trusted means to get
the same result. Perhaps compare output to a model of one aspect,
analogous function, other existing implementation, unoptimized code or
legacy code. Perhaps compare the output of old and new code to enable
reckless refactoring.

```rust
// Operation "n * 2" can be double checked with the oracle function "n + n",
// an analogous function to get to the same result.
use monkey_test::*;

monkey_test()
   .with_generator(gens::u8::ranged(..128))
   .assert_eq(|n| n + n, |n| n * 2 );
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
   .with_generator(gens::vec::any(gens::u8::any()))
   .assert_eq(
      |vec| vec.len() + 1,
      |mut vec| {
         vec.push(42 /* Add a single arbitrary item to vec */);
         vec.len()
      });
```

### Stateful testing

As one example, execute a series of commands against a stateful system, to
then verify some property of the system.

```rust,should_panic
// Make sure counter value is the same as the sum of all increments applied.
// In this case, it will fail with a series of increments like the shrunken
// failure example [-44, -128, -111, -128, -90] and failure
// reason "Actual value should equal expected -501, but got -500".

use monkey_test::*;

// The stateful system under test
struct Counter{acc: i64}
impl Counter {
   fn add(&mut self, value: i8) {
      self.acc += value as i64;
      // Buggy if statement...
      if self.acc < -500 {
         self.acc = -500;
      }
   }
}

monkey_test()
   .with_generator(gens::vec::any(gens::i8::any()))
   .assert_eq(
      // Expected sum of increments
      |vec| vec.iter().cloned().map(|x| x as i64).sum(),
      // Actual counter accumulation
      |vec| {
         let mut counter = Counter{ acc:0 };
         vec.iter().cloned().for_each(|x| counter.add(x));
         counter.acc
      });
```

Another example on stateful testing can be to poke at a stateful
API with a random sequence of legal commands and verify that the API does not
panic.
