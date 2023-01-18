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
        enum Test {
            TestV1(TestV1),
            TestV2(TestV2),
            TestV3(TestV3),
        }
        struct TestV1 {
            a: u64,
        }
        struct TestV2 {
            a: u64,
        }
        struct TestV3 {
            a: u64,
            b: u64,
        }
        impl TestV1 {
            fn new() -> Self {
                Self { a: 1 }
            }
        }
        impl TestV2 {
            fn new() -> Self {
                Self { a: 2 }
            }
        }
        impl TestV3 {
            fn new() -> Self {
                Self { a: 2, b: 3 }
            }
            fn get_b(&self) -> u64 {
                self.b
            }
        }
        impl TestV1 {
            fn get_a(&self) -> u64 {
                self.a
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }
        impl TestV2 {
            fn get_a(&self) -> u64 {
                self.a
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }
        let test = <TestV2>::new();
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
        let test = <TestV1>::new();
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
        let test = <TestV3>::new();
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
        impl Serializer<TestV1> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV1,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<TestV2> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV2,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<TestV3> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV3,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                buffer.push(data.b as u8);
                Ok(())
            }
        }
        struct TestDeserializer {}
        impl Deserializer<TestV1> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV1, E> {
                Ok((buffer, TestV1 { a: 2 }))
            }
        }
        impl Deserializer<TestV2> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV2, E> {
                Ok((buffer, TestV2 { a: 2 }))
            }
        }
        impl Deserializer<TestV3> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV3, E> {
                Ok((buffer, TestV3 { a: 2, b: 3 }))
            }
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&basic])
}
