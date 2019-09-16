use serde::{Deserialize, Serialize};
use serde_prefix::prefix_all;

#[prefix_all("test_")]
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[prefix_all("test_")]
#[derive(Serialize)]
enum TestEnum {
    Hello,
    Point { x: i32, y: i32 },
}

#[test]
fn test_prefix_struct() {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    let json = r#"{"test_x":1,"test_y":2}"#;
    assert_eq!(serialized, json);
}

#[test]
fn test_enum() {
    let serialized = serde_json::to_string(&TestEnum::Point { x: 1, y: 1 }).unwrap();
    let json = r#"{"test_Point":{"x":1,"y":1}}"#;
    assert_eq!(serialized, json);

    let serialized = serde_json::to_string(&TestEnum::Hello).unwrap();
    assert_eq!(serialized, "\"test_Hello\"");
}
