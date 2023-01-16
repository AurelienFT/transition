mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    use nom::{error::{ParseError, ContextError}, IResult};
    #[test]
    fn basic() {

        #[transition::versioned(versions("0.1.2", "0.2.0", "0.3.0"))]
        struct Test {
            a: u64,
            //TODO: Use add possibility to use <> in version number
            #[transition::field(versions("0.3.0"))]
            b: u64,
        }

        #[transition::impl_version(versions("0.1.2"))]
        impl Test {
            fn new() -> Self {
                Self { a: 1 }
            }
        }

        #[transition::impl_version(versions("0.2.0"))]
        impl Test {
            fn new() -> Self {
                Self { a: 2 }
            }
        }

        #[transition::impl_version(versions("0.3.0"))]
        impl Test {
            fn new() -> Self {
                Self { a: 2, b: 3 }
            }

            fn get_b(&self) -> u64 {
                self.b
            }
        }

        #[transition::impl_version(versions("0.1.2", "0.2.0"))]
        impl Test {
            fn get_a(&self) -> u64 {
                self.a
            }

            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }

        let test = <Test!["0.2.0"]>::new();
        assert_eq!(test.get_a(), 2);
        assert_eq!(test.mul(2), 4);

        let test = <Test!["0.1.2"]>::new();
        assert_eq!(test.get_a(), 1);
        assert_eq!(test.mul(2), 2);

        let test = <Test!["0.3.0"]>::new();
        assert_eq!(test.get_b(), 3);

        let test = Test::new();
        assert_eq!(test.get_b(), 3);

        //TODO: Add auto serialize version byte

        struct TestSerializer {}

        #[transition::impl_version(versions("0.1.2", "0.2.0"), structure = "Test")]
        impl Serializer<Test> for TestSerializer {
            fn serialize(&self, data: &Test, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }

        #[transition::impl_version(versions("0.3.0"), structure = "Test")]
        impl Serializer<Test> for TestSerializer {
            fn serialize(&self, data: &Test, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                buffer.push(data.b as u8);
                Ok(())
            }
        }

        struct TestDeserializer {}

        #[transition::impl_version(versions("0.1.2", "0.2.0"), structure = "Test")]
        impl Deserializer<Test> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                    &self,
                    buffer: &'a [u8],
                ) -> IResult<&'a [u8], Test, E> {
                    // Not really implemented because lack of time
                    Ok((buffer, Test {a: 2}))
            }
        }

        #[transition::impl_version(versions("0.3.0"), structure = "Test")]
        impl Deserializer<Test> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                    &self,
                    buffer: &'a [u8],
                ) -> IResult<&'a [u8], Test, E> {
                    // Compile error because `b` is not declared
                    //Ok((buffer, Test {a: 2}))
                    Ok((buffer, Test {a: 2, b: 3}))
            }
        }

    }
}