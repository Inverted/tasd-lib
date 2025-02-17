use tasd_lib_macro::Packet;

#[derive(Packet, Debug)]
#[key = 0x03]
pub struct GameTitle {
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tasd_lib_traits::Serializable;

    #[test]
    fn header_test() {
        let data = [
            0x00, 0x03, 0x01, 19, 83, 117, 112, 101, 114, 32, 77, 97, 114, 105, 111, 32, 66, 114,
            111, 115, 46, 32, 51,
        ];
        let (_, game_title) = GameTitle::deserialize(&data).unwrap();
        assert_eq!("Super Mario Bros. 3", game_title.title);
    }
}
