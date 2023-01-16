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

mod フォーマット {
    #[test]
    fn ゼロ埋めや空白埋めできる() {
        let x = "100";

        // 右寄せ8桁
        assert_eq!(format!("{x:>8} end"), "     100 end");
        // 左寄せ8桁
        assert_eq!(format!("{x:<8} end"), "100      end");
        // 中央寄せ8桁
        assert_eq!(format!("{x:^8} end"), "  100    end");
        // ゼロ埋め右寄せ8桁
        assert_eq!(format!("{x:0>8} end"), "00000100 end");
        // *埋め左寄せ8桁
        assert_eq!(format!("{x:*<8} end"), "100***** end");
    }

    #[test]
    fn 浮動小数点の精度_桁数を指定できる() {
        let x = 12.345;

        assert_eq!(format!("{x:.0}"), "12");
        // 四捨五入
        assert_eq!(format!("{x:.1}"), "12.3");
        // 四捨五入
        assert_eq!(format!("{x:.2}"), "12.35");
        assert_eq!(format!("{x:.3}"), "12.345");
        // 余った分は0で
        assert_eq!(format!("{x:.4}"), "12.3450");
    }
}
