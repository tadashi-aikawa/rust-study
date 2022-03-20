#![allow(non_snake_case)]

use std::fmt::Display;

use itertools::Itertools;
use maplit::hashmap;

fn error_msg<T: Display>(x: T) -> String {
    format!("Error: {}", x)
}

fn err<T: Display, R>(x: T) -> Result<R, String> {
    Err(error_msg(x))
}

fn result<T, F>(x: T, isOk: F) -> Result<T, String>
where
    T: Display,
    F: Fn(&T) -> bool,
{
    if isOk(&x) {
        Ok(x)
    } else {
        err(x)
    }
}

#[test]
fn vectorの作成() {
    let mut expected = Vec::new();
    expected.push(10);
    expected.push(20);
    expected.push(30);

    assert_eq!(expected, vec![10, 20, 30]);
}

#[test]
/// T[] -> U[]
fn 別の値に変換() {
    let actual = vec![10, 20, 30].iter().map(|x| x * 10).collect_vec();
    assert_eq!(vec![100, 200, 300], actual);
}

#[test]
/// T[] -> T[]
fn 条件にあう値のみをフィルタ() {
    let actual = vec![10, 20, 30]
        .into_iter()
        .filter(|&x| x > 10)
        .collect_vec();
    assert_eq!(vec![20, 30], actual);
}

#[test]
/// T[] -> Option[T]
fn 条件にあう最初の値を返す() {
    let actual = vec![10, 20, 30].into_iter().find(|&x| x > 20);
    assert_eq!(Some(30), actual);

    let actual = vec![10, 20, 30].into_iter().find(|&x| x > 30);
    assert_eq!(None, actual);
}

#[test]
/// T[] -> T[]
fn 有効な要素のみを抽出() {
    let actual = vec![Some(10), None, Some(30)]
        .into_iter()
        .flatten()
        .collect_vec();
    assert_eq!(vec![10, 30], actual);
}

#[test]
/// T[] -> T[]
fn 変換した結果が有効な要素のみを抽出() {
    let actual = vec![10, 20, 30]
        .iter()
        .flat_map(|&x| result(x, |&v| v > 10))
        .collect_vec();
    assert_eq!(vec![20, 30], actual);
}

#[test]
/// T[] -> Result<T[], E>
fn 変換に失敗した場合に最初に失敗した要素のエラーのみ返す() {
    let actual = vec![10, 20, 30, 40]
        .iter()
        .map(|&x| result(x, |&v| v < 30))
        .collect::<Result<Vec<_>, _>>();
    assert_eq!(err(30), actual);

    let actual = vec![10, 20]
        .iter()
        .map(|&x| result(x, |&v| v < 30))
        .collect::<Result<Vec<_>, _>>();
    assert_eq!(Ok(vec![10, 20]), actual);
}

#[test]
/// T[] -> Result<T[], E[]>
fn 変換に失敗した場合に失敗した全要素のエラーを返す() {
    fn fold_results(values: &[i32]) -> Result<Vec<i32>, Vec<String>> {
        let ret: (Vec<_>, Vec<_>) = values
            .iter()
            .map(|&x| result(x, |&v| v < 30))
            .partition_result();

        match ret {
            (oks, errs) if errs.is_empty() => Ok(oks),
            (_, errs) => Err(errs),
        }
    }

    assert_eq!(
        Err(vec![error_msg(30), error_msg(40)]),
        fold_results(&[10, 20, 30, 40])
    );
    assert_eq!(Ok(vec![10, 20]), fold_results(&[10, 20]));
}

#[test]
/// T[] -> T[]
fn ソート() {
    let actual = vec![15, 23, 31, 42, 54]
        .into_iter()
        .sorted_by_key(|x| x % 10)
        .collect_vec();
    assert_eq!(vec![31, 42, 23, 54, 15], actual);
}

#[test]
/// T[] -> T[]
fn 重複を削除() {
    let actual = vec![10, 20, 10, 30, 20].into_iter().unique().collect_vec();
    assert_eq!(vec![10, 20, 30], actual);
}

#[test]
/// T[] -> T[]
fn 条件を指定して重複を削除() {
    let actual = vec![11, 22, 12, 31, 22]
        .into_iter()
        .unique_by(|&x| x % 10)
        .collect_vec();
    assert_eq!(vec![11, 22], actual);
}

#[test]
/// T[] -> {K: t[]}
fn グルーピング() {
    let actual = vec![10, 11, 22, 30]
        .into_iter()
        .into_group_map_by(|&x| x % 10);
    assert_eq!(
        hashmap! {0 => vec![10, 30], 1 => vec![11], 2 => vec![22]},
        actual
    );
}

#[test]
/// T[] -> {K: usize}
fn グルーピングしてカウント() {
    let actual = vec![10, 11, 22, 30].iter().counts_by(|&x| x % 10);
    assert_eq!(hashmap! {0 => 2, 1 => 1, 2 => 1}, actual);
}

#[test]
/// T[] -> U
fn 初期値を指定して畳み込み() {
    let actual = vec![10, 20, 30].into_iter().fold(100, |a, b| a - b);
    assert_eq!(40, actual);
}

#[test]
/// T[] -> U
fn 最初の要素を初期値として畳み込み() {
    let actual = vec![10, 20, 30].into_iter().reduce(|a, b| a - b);
    assert_eq!(Some(-40), actual);
}

#[test]
/// T[] -> T[]
fn 並びを逆転() {
    let actual = vec![10, 20, 30].into_iter().rev().collect_vec();
    assert_eq!(vec![30, 20, 10], actual);
}
