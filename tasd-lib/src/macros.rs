#[macro_export]
macro_rules! create_parsers {
    ($( $variant: ident ),*) => {
        #[derive(Debug)]
        pub enum Packet {
            $( $variant($variant), )*
        }

        impl Packet {
            pub fn serialize(&self) -> Vec<u8> {
                match self {
                    $(Packet::$variant(data) => data.serialize(),)+
                }
            }

            fn deserialize(input: &[u8]) -> IResult<&[u8], Self>
            where
                Self: Sized,
            {
                let parsers = vec![
                    $(Box::new($variant::to_packet) as Box<dyn Fn(&[u8]) -> IResult<&[u8], Packet>>),+
                ];

                for parser in parsers {
                    match parser(input) {
                        Ok((new_input, packet)) => return Ok((new_input, packet)),
                        Err(_) => continue, // Properly handle or log the error as needed
                    }
                }

                // If none of the parsers succeeded, return an appropriate error
                Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))
            }

            fn size(&self) -> u64 {
                 match self {
                    $(Packet::$variant(data) => data.size(),)+
                }
            }
        }

        $(impl $variant {
            fn to_packet(input: &[u8]) -> nom::IResult<&[u8], Packet> {
                let (input, result) = $variant::deserialize(input)?;
                Ok((input, Packet::$variant(result)))
            }
        })*
    };
}
