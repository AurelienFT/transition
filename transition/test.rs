#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
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
        struct Test {
            a: u64,
        }
        struct Test_V0_1_2 {
            a: u64,
        }
        struct Test_V0_2_0 {
            a: u64,
        }
        struct Test_V0_3_0 {
            a: u64,
        }
        impl Test_V0_1_2 {
            fn new() -> Self {
                Self { a: 1 }
            }
        }
        impl Test_V0_2_0 {
            fn new() -> Self {
                Self { a: 2 }
            }
        }
        impl Test_V0_1_2 {
            fn get_a(&self) -> u64 {
                self.a
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }
        impl Test_V0_2_0 {
            fn get_a(&self) -> u64 {
                self.a
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }
        let test = <Test_V0_2_0>::new();
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
        let test = <Test_V0_1_2>::new();
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
        struct TestSerializer {}
        impl Serializer<Test_V0_1_2> for TestSerializer {
            fn serialize(
                &self,
                data: &Test_V0_1_2,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<Test_V0_2_0> for TestSerializer {
            fn serialize(
                &self,
                data: &Test_V0_2_0,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<Test_V0_3_0> for TestSerializer {
            fn serialize(
                &self,
                data: &Test_V0_3_0,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&basic])
}
