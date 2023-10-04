use serde_django::model;

#[model]
struct MyModel {
    field1: i32,
    field2: String,
    // ...
}

fn main() {
    // ...
}
