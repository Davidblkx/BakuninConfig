extern crate bakunin_config;

use bakunin_config::{Value, value_map, value_vec};

fn main() {
    let val = value_map! {
        prop1: "value1",
        prop2: 10,
        prop3: value_vec![1, 2, 3],
        prop4: value_map! {
            prop5: "value2"
        }
    };

    // The `get` method returns a reference to the value of a property and the `try_into` method can be used to convert it to a specific type
    let prop1: String = val.get("prop1").try_into().unwrap();
    assert_eq!(prop1, "value1");

    // It's also possible to use the `try_into_<type>` method to convert a value to a specific type
    let prop2 = val.get("prop2").try_into_f64().unwrap();
    assert_eq!(prop2, 10.0);

    // If the value is a vector, it's possible to iterate over it
    let mut sum = 0;
    for v in val.get("prop3").iter() {
        let v: i32 = v.try_into().unwrap();
        sum += v;
    }
    assert_eq!(sum, 6);

    // If the value is a map, it's possible to access its properties and sub-properties
    let prop5 = val.get("prop4").get("prop5").try_into_string().unwrap();
    assert_eq!(prop5, "value2");

    // If the property does not exist, the `get` method returns a `None` value
    let prop6 = val.get("prop6").try_into_string().ok();
    assert_eq!(prop6, None);
}