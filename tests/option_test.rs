#![allow(non_snake_case)]

use itertools::Itertools;

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
