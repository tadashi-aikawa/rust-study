#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Dog {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct Cat {
    id: i32,
    name: String,
}

enum Animal {
    #[allow(dead_code)]
    Dog(Dog),
    Cat(Cat),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Lang {
    Ja,
    En,
}
impl Display for Lang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = serde_lexpr::to_string(self).unwrap();
        write!(f, "{}", x)
    }
}


mod シンプル {
    use super::*;

    #[test]
    fn 構造体を外部定義にしてenumできる() {
        fn get_identifier(animal: Animal) -> String {
            match animal {
                Animal::Dog(dog) => format!("{}: {}", dog.id, dog.name),
                Animal::Cat(cat) => format!("{}: {}", cat.id, cat.name),
            }
        }

        let cat = Cat {
            id: 10,
            name: "mike".to_string(),
        };
        assert_eq!(get_identifier(Animal::Cat(cat)), "10: mike".to_string());
    }

    #[test]
    fn enumを文字列として出力できる() {
        let actual = format!("{}", Lang::Ja);
        assert_eq!("ja", actual);
    }
}
