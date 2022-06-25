#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Cat {
    id: i32,
    name: String,
}
impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id = {}, name = {}", self.id, self.name)
    }
}

mod シンプル {
    use super::*;

    #[test]
    fn formatで出力できる() {
        let cat = Cat {
            id: 10,
            name: "mike".to_string(),
        };

        let actual = format!("{}", cat);
        assert_eq!("id = 10, name = mike", actual);
    }
}