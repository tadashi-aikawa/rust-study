#![allow(non_snake_case)]

#[test]
fn Optionの中身を実体化() {
    let s1 = &"aaa".to_string();

    let seed = Some(s1);
    let actual = seed.cloned();

    assert_eq!(Some("aaa".to_string()), actual);
}

#[test]
fn Optionの中身をClone() {
    let s1 = Some("aaa".to_string());
    let b = s1.as_ref().cloned().unwrap();
    let c = s1.unwrap();

    assert_eq!(b, c);
}

#[test]
fn OptionをResultに変換() {
    let actual_some = Some("some").ok_or("error");
    assert_eq!(Ok("some"), actual_some);

    let actual_none: Result<&str, &str> = None.ok_or("error");
    assert_eq!(Err("error"), actual_none);
}
