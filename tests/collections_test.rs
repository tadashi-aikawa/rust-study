#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::iter;
use std::ops::Not;

use itertools::Itertools;
use maplit::{hashmap, hashset};

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
fn rangeの作成() {
    let range = 0..5;

    let actual = range.collect::<Vec<_>>();
    assert_eq!(vec![0, 1, 2, 3, 4], actual);
}

#[test]
fn index付きiteratorの作成() {
    let values = vec!["aaa", "iii", "uuu"];

    let actual = values
        .iter()
        .enumerate()
        .map(|(i, v)| format!("{}: {}", i, v))
        .collect::<Vec<_>>();
    assert_eq!(vec!["0: aaa", "1: iii", "2: uuu"], actual);
}

#[test]
fn 同じ値を繰り返すIteratorを作成() {
    let endless_tadashi = iter::repeat("tadashi");

    let actual = endless_tadashi.take(3).collect_vec();
    assert_eq!(vec!["tadashi", "tadashi", "tadashi"], actual);
}

#[test]
fn sliceをvectorに変換() {
    let s = &[1, 2, 3];
    let actual = s.to_vec();
    assert_eq!(vec![1, 2, 3], actual);
}

#[test]
fn vectorをsliceに変換() {
    let v = vec![1, 2, 3];
    let actual = v.as_slice();
    assert_eq!(&[1, 2, 3], actual);
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
fn 有効値の中身のみをフィルタ() {
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
/// T[] -> {K: T}
fn インデクシング() {
    let actual = vec![10, 11, 22, 30]
        .into_iter()
        .fold(HashMap::new(), |mut acc, c| {
            acc.insert(c % 10, c);
            acc
        });
    assert_eq!(hashmap! {0 => 30, 1 => 11, 2 => 22}, actual);
}

#[test]
/// T[] -> {K: T[]}
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
/// T[] -> T
fn 最大値の取得() {
    let actual = vec![30, 10, 20].into_iter().max();
    assert_eq!(Some(30), actual);
}

#[test]
/// T[] -> T
fn 最小値の取得() {
    let actual = vec![30, 10, 20].into_iter().min();
    assert_eq!(Some(10), actual);
}

#[test]
/// T[] -> T
fn 変換結果が最大となる値の取得() {
    let actual = vec![51, 43, 32].into_iter().max_by_key(|x| x % 10);
    assert_eq!(Some(43), actual);
}

#[test]
/// T[] -> T
fn 変換結果が最小となる値の取得() {
    let actual = vec![51, 43, 32].into_iter().min_by_key(|x| x % 10);
    assert_eq!(Some(51), actual);
}

#[test]
/// T[] -> T[]
fn 並びを逆転() {
    let actual = vec![10, 20, 30].into_iter().rev().collect_vec();
    assert_eq!(vec![30, 20, 10], actual);
}

#[test]
/// T[] -> T[]
fn Vectorの結合_mutable() {
    let mut seed = vec![10, 20, 30];
    seed.extend(vec![40, 50]);
    let actual = seed;

    assert_eq!(vec![10, 20, 30, 40, 50], actual);
}

#[test]
/// T[] -> T[]
fn Vectorの結合_immutable() {
    let seed1 = vec![10, 20, 30];
    let seed2 = vec![40, 50];

    // seed1[..] -> [i32]
    // &seed1[..] -> &[i32]
    // [&seed1[..], &seed2[..]] -> [&[i32]; 2]
    let actual = [&seed1[..], &seed2[..]].concat();

    assert_eq!(vec![10, 20, 30, 40, 50], actual);
}

#[test]
/// (T, T[]) -> T[]
fn 値とVectorの結合() {
    let seed1 = 10;
    let seed2 = vec![20, 30];

    // seed1[..] -> [i32]
    // &seed1[..] -> &[i32]
    // [&seed1[..], &seed2[..]] -> [&[i32]; 2]
    let actual = [&[seed1], &seed2[..]].concat();

    assert_eq!(vec![10, 20, 30], actual);
}

#[test]
fn Vecの中身を実体化() {
    let s1 = &"aaa".to_string();
    let s2 = &"bbb".to_string();
    let actual = vec![s1, s2].into_iter().cloned().collect_vec();

    assert_eq!(vec!["aaa".to_string(), "bbb".to_string()], actual);
}

#[test]
fn Vecのクローン() {
    let xs = vec!["a", "b"];
    let ys = xs.to_vec();

    assert_eq!(xs, ys);
}

#[test]
fn VecをOption_Vecに変換() {
    let xs = vec!["a"];
    let actual1 = xs.is_empty().not().then_some(xs);
    assert_eq!(Some(vec!["a"]), actual1);

    let empty: Vec<&str> = vec![];
    let actual1 = empty.is_empty().not().then_some(empty);
    assert_eq!(None, actual1);
}

#[test]
fn VecをSetとして重複削除() {
    let xs = vec!["a", "b", "a", "c", "b", "a"];
    assert_eq!(
        hashset!["a", "b", "c"],
        xs.into_iter().collect::<HashSet<_>>()
    );
}
