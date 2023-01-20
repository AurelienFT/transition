mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    use nom::{error::{ParseError, ContextError}, IResult};
    //TODO: Make it optional
    use transition::Versioned;
    #[test]
    fn enum_test() {

        #[transition::versioned_enum(versions("1", "2", "3"))]
        enum Human {
            Alice(Alice),
            Bob(Bob),
        }

        struct Alice {
            a: u64,
        }

        struct Bob {
            b: u64,
        }

    }
}