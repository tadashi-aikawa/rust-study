#![allow(non_snake_case)]

extern crate core;

use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

#[derive(Primitive, PartialEq, Debug)]
enum Number {
    One = 1,
    Ten = 10,
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

    #[test]
    fn i32をenumに変換できる() {
        let actual: Option<Number> = Number::from_i32(10);
        assert_eq!(Some(Number::Ten), actual);
    }
}
