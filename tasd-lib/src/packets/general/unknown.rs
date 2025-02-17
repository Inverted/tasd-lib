use nom::number::streaming::be_u16;
use nom::IResult;
use tasd_lib_traits::Serializable;

#[derive(Debug)]
pub struct Unknown {
    pub key: u16,
    pub payload: Vec<u8>,
}

impl Serializable for Unknown {
    fn size(&self) -> u64 {
        return self.payload.len() as u64;
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend((self.key).to_be_bytes());
        bytes.extend(&self.payload);

        bytes
    }
    fn deserialize(input: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (input, key) = be_u16(input)?;
        let (input, p_exp) = nom::number::streaming::be_u8(input)?;
        let (input, p_len) = nom::bytes::streaming::take(p_exp)(input)?;
        let mut p_len_dst: [u8; 8] = [0; 8];
        p_len_dst[(8 - p_len.len())..8].copy_from_slice(p_len);
        let p_len: u64 = u64::from_be_bytes(p_len_dst);
        let (input, payload) = nom::bytes::streaming::take(p_len)(input)?;
        Ok((
            input,
            Unknown {
                key,
                payload: payload.to_vec(),
            },
        ))
    }
}
