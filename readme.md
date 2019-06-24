[![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://travis-ci.org/jonathan-s/serde-prefix.svg?branch=master
[travis]: https://travis-ci.org/jonathan-s/serde-prefix

[Latest Version]: https://img.shields.io/crates/v/serde_prefix.svg
[crates.io]: https://crates.io/crates/serde_prefix

# Serde Prefix

A small extension to serde that will allow you to use the macro `#[prefix_all("myprefix_")`. The macro will prefix each attribute in a struct or enum with the prefix of your choice. 

Behind the doors it's using `#[serde(rename = "...")]` to rename each attribute with the prefix defined in prefix_all. 

## Usage

```rust
#[macro_use]
extern crate serde_prefix;
extern crate serde;

use serde::{Serialize, Deserialize};


#[prefix_all("test_")]
#[derive(Serialize, Debug)]
struct Point {
    x: i32,
    y: i32
}


let point = Point { x: 1, y: 2 };
let serialized = serde_json::to_string(&point).unwrap();
let json = r#"{"test_x":1,"test_y":2}"#;
assert_eq!(serialized, json);
```

If there is anything that you are missing create an issue :). 
