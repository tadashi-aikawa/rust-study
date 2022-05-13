#![allow(non_snake_case)]

/// https://serde.rs/derive.html
mod 構造体をJSONにパース_Serialize {
    use serde::{Deserialize, Serialize};

    #[test]
    fn シンプル() {
        #[derive(Serialize, Deserialize, Debug)]
        struct Human {
            id: i32,
            name: String,
        }

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
    /// https://serde.rs/field-attrs.html#rename
    fn フィールド名が変わる() {
        #[derive(Serialize, Deserialize, Debug)]
        struct Human {
            id: i32,
            #[serde(rename = "full_name")]
            name: String,
        }

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
    /// https://serde.rs/field-attrs.html#skip
    fn フィールドを省略する() {
        #[derive(Serialize, Deserialize, Debug)]
        struct Human {
            id: i32,
            #[serde(skip)]
            name: String,
        }

        let target = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(r#"{"id":1}"#, serde_json::to_string(&target).unwrap());
        assert_eq!("Ichiro", target.name);
    }
}

/// https://serde.rs/derive.html
mod JSONを構造体にパース_Deserialize {
    use serde::{Deserialize, Serialize};

    #[test]
    fn シンプル() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Human {
            id: i32,
            name: String,
        }

        let expected = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"name":"Ichiro"}"#).unwrap()
        );
    }

    #[test]
    /// https://serde.rs/field-attrs.html#rename
    fn フィールド名が変わる() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Human {
            id: i32,
            #[serde(rename = "full_name")]
            name: String,
        }

        let expected = Human {
            id: 1,
            name: "Ichiro".to_string(),
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"full_name":"Ichiro"}"#).unwrap()
        );
    }

    #[test]
    /// https://serde.rs/field-attrs.html#rename
    fn フィールド名に複数の別名がある() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Human {
            id: i32,
            #[serde(alias = "full_name")]
            #[serde(alias = "nm")]
            name: String,
        }

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

    #[test]
    /// https://serde.rs/field-attrs.html#skip
    fn フィールドを省略する() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Human {
            id: i32,
            #[serde(skip)]
            name: String,
            #[serde(skip)]
            optional_name: Option<String>,
        }

        let expected = Human {
            id: 1,
            name: "".to_string(),
            optional_name: None,
        };
        assert_eq!(
            expected,
            serde_json::from_str(r#"{"id":1,"name":"Ichiro","optional_name":"Jiro"}"#).unwrap()
        );
    }
}
