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

#[derive(Debug)]
struct Logger {
    logs: Vec<String>,
}
impl Logger {
    fn new() -> Self {
        Logger { logs: vec![] }
    }

    fn append(&mut self, log: String) {
        self.logs.push(log);
    }

    fn show(&self) {
        println!("{}", self.logs.join("\n"));
    }
}

mod ロガーの挙動 {
    use super::*;

    #[test]
    fn ロガーにログを2つ以上追加できる() {
        let mut logger = Logger::new();
        logger.append("log1".to_string());
        logger.append("log2".to_string());
        logger.show();
    }

    #[test]
    fn クロージャーの内外でログを2つ以上追加できる() {
        let mut logger = Logger::new();
        logger.append("log1".to_string());
        vec![String::from("AAA"), String::from("BBB")]
            .into_iter()
            .for_each(|x| {
                logger.append(x);
            });
        logger.show();
    }
}
