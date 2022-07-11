#![allow(non_snake_case)]

mod 文字列の変換 {
    #[test]
    fn 部分文字列をスライスで分割できる() {
        let string = "aabbcc".to_string();

        assert_eq!("aa", &string[..2]);
        assert_eq!("bb", &string[2..4]);
        assert_eq!("cc", &string[4..]);
    }
}
