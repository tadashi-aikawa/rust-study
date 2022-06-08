#![allow(non_snake_case)]

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

mod シンプル {
    use super::*;

    #[test]
    fn スラッシュのコメント行がコメント行としてマッチできる() {
        assert_eq!(from("// hoge"), LineType::SlashComment);
        assert_eq!(from("//hoge"), LineType::SlashComment);
        assert_eq!(from("# hoge"), LineType::SharpComment);
        assert_eq!(from("#hoge"), LineType::SharpComment);
        assert_eq!(from("/ hoge"), LineType::Text);
    }
}
