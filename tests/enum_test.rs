#![allow(non_snake_case)]

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
    Dog(Dog),
    Cat(Cat),
}

mod シンプル {
    use super::*;
    use std::fmt::format;

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
        assert_eq!(get_identifier(Animal::Cat(cat)), "mike".to_string());
    }
}
