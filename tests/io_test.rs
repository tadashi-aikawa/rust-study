#![allow(non_snake_case)]

mod シンプル {
    use std::fs::File;
    use std::io;
    use std::io::{BufWriter, Read, Write};

    #[test]
    fn Writeを使ってVecに書き込み() -> io::Result<()> {
        let mut out = Vec::new();
        let _result = out.write_all("hoge".to_string().as_ref())?;
        assert_eq!("hoge", String::from_utf8(out).unwrap());
        Ok(())
    }

    #[test]
    fn Writeを使って標準出力に書き込み() -> io::Result<()> {
        let mut out = std::io::stdout();
        let _result = out.write_all("hoge".to_string().as_ref())?;
        Ok(())
    }

    #[test]
    fn Writeを使ってファイルに書き込み() -> io::Result<()> {
        let mut out = File::create("./result.txt")?;
        let _result = out.write_all("hoge".to_string().as_ref())?;
        let expected = std::fs::read_to_string("./result.txt")?;

        assert_eq!(expected, "hoge");
        Ok(())
    }
}
