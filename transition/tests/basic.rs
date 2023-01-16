mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    #[test]
    fn basic() {

        #[transition::versioned(versions("0.1.2", "0.2.0", "0.3.0"))]
        struct Test {
            a: u64,
           // #[transition::versioned(versions("0.3.0"))]
           // b: u64,
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

        #[transition::impl_version(versions("0.1.2", "0.2.0"))]
        impl Test {
            fn get_a(&self) -> u64 {
                self.a
            }

            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
        }

        //TODO: Add syntax without specifying version for default to use the latest. Do we really want this ? 

        let test = <Test!["0.2.0"]>::new();
        assert_eq!(test.get_a(), 2);

        let test = <Test!["0.1.2"]>::new();
        assert_eq!(test.get_a(), 1);

        assert_eq!(test.mul(2), 2);

        struct TestSerializer {}

        //TODO: Make it works to be able to use the same serializer for multiple versions and don't use the macro Test!
        //#[transition::impl_version(versions("0.1.2", "0.2.0"), structure = "Test")]
        impl Serializer<Test!["0.2.0"]> for TestSerializer {
            fn serialize(&self, data: &Test!["0.2.0"], buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }

        //TODO: Make it works to be able to use the same serializer for multiple versions and don't use the macro Test!
        //#[transition::impl_version(versions("0.3.0"), structure = "Test")]
        impl Serializer<Test!["0.3.0"]> for TestSerializer {
            fn serialize(&self, data: &Test!["0.3.0"], buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                //buffer.push(data.b as u8);
                Ok(())
            }
        }

        //TODO: Add deserializer
    }
}