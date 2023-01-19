# transition

**CURRENTLY WIP**

Transition is a macro to version your structures and methods

The goal of this crate is to be able to versions structures and so have fields or `impl` that correspond to different version of the structure.

This crate takes inspiration from : https://github.com/doctorn/obake

## Example with explanations: 

```rust
// Declare a structure to be versioned. This structure will become an enum with 3 variants for the 3 versions
#[transition::versioned(versions("1", "2", "3"))]
struct Test {
    a: u64,
    // Declare a field to be included only in this version
    #[transition::field(versions("3"))]
    b: u64,
}

// Declare common methods that will interact with methods of each variant
impl Test {
    fn new(version: u64) -> Self {
        match version {
            <Test!["1"]>::VERSION => TestVariant!["1"](<Test!["1"]>::new()),
            <Test!["2"]>::VERSION => TestVariant!["2"](<Test!["2"]>::new()),
            <Test!["3"]>::VERSION => TestVariant!["3"](<Test!["3"]>::new()),
            _ => panic!("Unknown version: {}", version)
        }
    }

    fn get_a(&self) -> u64 {
        match self {
            Test::TestV1(test) => test.get_a(),
            Test::TestV2(test) => test.get_a(),
            Test::TestV3(test) => test.get_a(),
        }
    }
}

// Declare a implementation of methods only for version 1
#[transition::impl_version(versions("1"))]
impl Test {
    fn new() -> Self {
        Self { a: 1 }
    }
}

// Declare a implementation of methods only for version 2
#[transition::impl_version(versions("2"))]
impl Test {
    fn new() -> Self {
        Self { a: 2 }
    }
}

// Declare a implementation of methods only for version 3
#[transition::impl_version(versions("3"))]
impl Test {
    fn new() -> Self {
        Self { a: 2, b: 3 }
    }

    // Only version 3 has this method because only this version has the field `b`
    fn get_b(&self) -> u64 {
        self.b
    }
}

// Implementation that is working for all versions
#[transition::impl_version(versions("1", "2", "3"))]
impl Test {
    fn get_a(&self) -> u64 {
        self.a
    }

    fn mul(&self, b: u64) -> u64 {
        self.a * b
    }
}

// Declare a `Test` structure with a known version
let test = <Test!["2"]>::new();
assert_eq!(test.get_a(), 2);
assert_eq!(test.mul(2), 4);

// Declare a `Test` structure with a known version
let test = <Test!["1"]>::new();
assert_eq!(test.get_a(), 1);
assert_eq!(test.mul(2), 2);

// Declare a `Test` structure with a known version
let test = <Test!["3"]>::new();
assert_eq!(test.get_b(), 3);

// Declare a `Test` structure with a variable version, so the version possibly can't be determine at compile time
// and so it return a enum and you are able to use the common methods
let test = Test::new(1);
assert_eq!(test.get_a(), 1);
```

This example with even more usage is available [here](https://github.com/AurelienFT/transition/blob/main/transition/tests/basic.rs) and the expanded code is located [here](https://github.com/AurelienFT/transition/blob/main/transition/test.rs)