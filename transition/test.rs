#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    use nom::{
        error::{ParseError, ContextError},
        IResult,
    };
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::basic"]
    pub const basic: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::basic"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(basic())),
    };
    fn basic() {
        type Test = TestV0_3_0;
        struct TestV0_1_2 {
            a: u64,
        }
        struct TestV0_2_0 {
            a: u64,
        }
        struct TestV0_3_0 {
            a: u64,
            b: u64,
        }
        impl TestV0_1_2 {
            fn new() -> Self {
                Self { a: 1 }
            }
        }
        impl TestV0_2_0 {
            fn new() -> Self {
                Self { a: 2 }
            }
        }
        impl TestV0_3_0 {
            fn new() -> Self {
                Self { a: 2, b: 3 }
            }
            fn get_b(&self) -> u64 {
                self.b
            }
        }
        impl TestV0_1_2 {
            fn get_a(&self) -> u64 {
                self.a
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }
        impl TestV0_2_0 {
            fn get_a(&self) -> u64 {
                self.a
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }
        let test = <TestV0_2_0>::new();
        match (&test.get_a(), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&test.mul(2), &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let test = <TestV0_1_2>::new();
        match (&test.get_a(), &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&test.mul(2), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let test = <TestV0_3_0>::new();
        match (&test.get_b(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let test = Test::new();
        match (&test.get_b(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        struct TestSerializer {}
        impl Serializer<TestV0_1_2> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV0_1_2,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<TestV0_2_0> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV0_2_0,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<TestV0_3_0> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV0_3_0,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                buffer.push(data.b as u8);
                Ok(())
            }
        }
        struct TestDeserializer {}
        impl Deserializer<TestV0_1_2> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV0_1_2, E> {
                Ok((buffer, TestV0_1_2 { a: 2 }))
            }
        }
        impl Deserializer<TestV0_2_0> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV0_2_0, E> {
                Ok((buffer, TestV0_2_0 { a: 2 }))
            }
        }
        impl Deserializer<TestV0_3_0> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV0_3_0, E> {
                Ok((buffer, TestV0_3_0 { a: 2, b: 3 }))
            }
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&basic])
}
