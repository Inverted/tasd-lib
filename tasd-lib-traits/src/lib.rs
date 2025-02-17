pub trait Serializable {
    fn size(&self) -> u64;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized;
}
