#![allow(non_snake_case)]

extern crate core;

mod 行とパターンマッチ {
    #[derive(PartialEq, Debug)]
    enum LineType {
        SharpComment,
        SlashComment,
        Text,
    }

    fn from(str: &str) -> LineType {
        let first = str.get(..1);
        let second = str.get(1..2);
        match (first, second) {
            (Some("/"), Some("/")) => LineType::SlashComment,
            (Some("#"), _) => LineType::SharpComment,
            _ => LineType::Text,
        }
    }

    #[test]
    fn コメント行がコメント行としてマッチできる() {
        assert_eq!(from("// hoge"), LineType::SlashComment);
        assert_eq!(from("//hoge"), LineType::SlashComment);
        assert_eq!(from("# hoge"), LineType::SharpComment);
        assert_eq!(from("#hoge"), LineType::SharpComment);
        assert_eq!(from("/ hoge"), LineType::Text);
    }
}

mod Vectorの要素数とパターンマッチ {
    use itertools::Itertools;

    #[derive(PartialEq, Debug)]
    enum VectorType {
        Empty,
        Only(String),
        Multi(String, Vec<String>),
    }

    fn from(strs: &[String]) -> VectorType {
        match strs {
            [] => VectorType::Empty,
            [first] => VectorType::Only(first.to_string()),
            [first, rest @ ..] => {
                VectorType::Multi(first.to_string(), rest.iter().cloned().collect_vec())
            }
        }
    }

    #[test]
    fn Vectorの要素数でパターンマッチできる() {
        assert_eq!(from(&[]), VectorType::Empty);
        assert_eq!(from(&["a".to_string()]), VectorType::Only("a".to_string()));
        assert_eq!(
            from(&["a".to_string(), "b".to_string()]),
            VectorType::Multi("a".to_string(), vec!["b".to_string()]),
        );
        assert_eq!(
            from(&["a".to_string(), "b".to_string(), "c".to_string()]),
            VectorType::Multi("a".to_string(), vec!["b".to_string(), "c".to_string()]),
        );
    }
}
