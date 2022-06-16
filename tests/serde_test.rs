#![allow(non_snake_case)]

mod シンプル {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Human {
        id: i32,
        name: String,
    }

    #[test]
    fn 構造体をJSONにserialize() {
        let target = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(
            r#"{"id":1,"name":"Ichiro"}"#,
            serde_json::to_string(&target).unwrap(),
        );
    }

    #[test]
    fn JSONを構造体にdeserialize() {
        let expected = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"name":"Ichiro"}"#).unwrap()
        );
    }
}

/// https://serde.rs/field-attrs.html#rename
mod フィールド名が変わる {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Human {
        id: i32,
        #[serde(rename = "full_name")]
        name: String,
    }

    #[test]
    fn 構造体をJSONにserialize() {
        let target = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(
            r#"{"id":1,"full_name":"Ichiro"}"#,
            serde_json::to_string(&target).unwrap(),
        );
    }

    #[test]
    fn JSONを構造体にdeserialize() {
        let expected = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"full_name":"Ichiro"}"#).unwrap()
        );
    }
}

mod フィールドに複数の別名がある {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Human {
        id: i32,
        #[serde(alias = "full_name")]
        #[serde(alias = "nm")]
        name: String,
    }

    #[test]
    fn JSONを構造体にdeserialize() {
        let expected = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"full_name":"Ichiro"}"#).unwrap()
        );
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"nm":"Ichiro"}"#).unwrap()
        );
    }
}

/// https://serde.rs/field-attrs.html#skip
mod フィールドを省略する {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Human {
        id: i32,
        #[serde(skip)]
        name: String,
    }

    #[test]
    fn 構造体をJSONにserialize() {
        let target = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(r#"{"id":1}"#, serde_json::to_string(&target).unwrap(),);
    }

    #[test]
    fn JSONを構造体にdeserialize() {
        let expected = Human {
            id: 1,
            name: "".to_string(),
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"full_name":"Ichiro"}"#).unwrap()
        );
    }
}

mod Enum {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Family {
        pet1: Animal,
        pet2: Animal,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum Animal {
        Dog,
        #[serde(rename = "nyan")]
        Cat,
    }

    #[test]
    fn 構造体をJSONにserialize() {
        let target = Family {
            pet1: Animal::Dog,
            pet2: Animal::Cat,
        };

        assert_eq!(
            r#"{"pet1":"Dog","pet2":"nyan"}"#,
            serde_json::to_string(&target).unwrap(),
        );
    }

    #[test]
    fn JSONを構造体にdeserialize() {
        let expected = Family {
            pet1: Animal::Dog,
            pet2: Animal::Cat,
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"pet1":"Dog","pet2":"nyan"}"#).unwrap()
        );
    }

    #[test]
    fn Variantをstringにserialize() {
        let expected = "nyan";
        assert_eq!(expected, serde_lexpr::to_string(&Animal::Cat).unwrap());
    }

    #[test]
    fn stringをVariantにdeserialize() {
        let expected = Animal::Cat;
        assert_eq!(expected, serde_lexpr::from_str("nyan").unwrap());
    }
}
