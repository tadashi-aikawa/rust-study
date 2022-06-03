#![allow(non_snake_case)]

#[derive(Default)]
struct User {
    id: i32,
    name: String,
}

struct Group {
    users: Vec<User>,
}
impl Default for Group {
    fn default() -> Self {
        Group {
            users: vec![User {
                id: 1,
                name: "tadashi-aikawa".to_string(),
            }],
        }
    }
}

#[derive(Default)]
struct Pair {
    user1: User,
    user2: User,
}

mod シンプル {
    use super::*;

    #[test]
    fn Defaultトレイトを実装した構造体はdefaultでデフォルト値を生成できる() {
        let actual = User::default();
        assert_eq!(actual.id, 0);
        assert_eq!(actual.name, "");
    }

    #[test]
    fn デフォルト値を上書きできる() {
        let actual = User {
            name: "tadashi-aikawa".to_string(),
            ..User::default()
        };
        assert_eq!(actual.id, 0);
        assert_eq!(actual.name, "tadashi-aikawa");
    }

    #[test]
    fn デフォルト値を指定して生成できる() {
        let actual = Group::default();
        assert_eq!(actual.users[0].id, 1);
        assert_eq!(actual.users[0].name, "tadashi-aikawa");
    }

    #[test]
    fn デフォルト値をネストして生成できる() {
        let actual = Pair::default();
        assert_eq!(actual.user1.id, 0);
        assert_eq!(actual.user2.id, 0);
    }
}
