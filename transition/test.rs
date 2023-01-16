0.1.2 => Test_V0_1_2
0.2.0 => Test_V0_2_0
[TokenStream [Group { delimiter: Bracket, stream: TokenStream [Literal { kind: Str, symbol: "0.1.2", suffix: None, span: #7 bytes(26..78) }], span: #7 bytes(26..78) }, Punct { ch: '=', spacing: Joint, span: #7 bytes(26..78) }, Punct { ch: '>', spacing: Alone, span: #7 bytes(26..78) }, Group { delimiter: Brace, stream: TokenStream [Ident { ident: "Test_V0_1_2", span: #0 bytes(90..94) }], span: #7 bytes(26..78) }, Punct { ch: ';', spacing: Alone, span: #7 bytes(26..78) }], TokenStream [Group { delimiter: Bracket, stream: TokenStream [Literal { kind: Str, symbol: "0.2.0", suffix: None, span: #7 bytes(26..78) }], span: #7 bytes(26..78) }, Punct { ch: '=', spacing: Joint, span: #7 bytes(26..78) }, Punct { ch: '>', spacing: Alone, span: #7 bytes(26..78) }, Group { delimiter: Brace, stream: TokenStream [Ident { ident: "Test_V0_2_0", span: #0 bytes(90..94) }], span: #7 bytes(26..78) }, Punct { ch: ';', spacing: Alone, span: #7 bytes(26..78) }]]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "basic"]
pub const basic: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("basic"),
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
    let test = <()>::new();
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&test.get_a())],
            ),
        );
    };
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&basic])
}
