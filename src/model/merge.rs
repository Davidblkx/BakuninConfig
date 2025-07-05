use super::Value;

impl Value {
    pub fn merge(&mut self, other: &Value) {
        match (self, other) {
            (Value::Map(lhs), Value::Map(rhs)) => {
                for (key, value) in rhs {
                    match lhs.get_mut(key) {
                        Some(lhs_value) => {
                            if lhs_value.is_map() && value.is_map() {
                                lhs_value.merge(value);
                            } else {
                                lhs.insert(key.clone(), value.clone());
                            }
                        }
                        None => {
                            lhs.insert(key.clone(), value.clone());
                        }
                    }
                }
            },
            _ => {}
        }
    }

    pub fn clone_merge(&self, other: &Value) -> Value {
        let mut value = self.clone();
        value.merge(other);
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::value_map;

    #[test]
    fn test_merge() {
        let mut value = value_map!{
            name: "David",
            age: 33,
            address: value_map!{
                street: "123 Main St",
                city: "Anytown",
                state: "CA",
                zip: 12345,
            },
        };

        let local = value_map!{
            address: value_map!{
                street: "123 Main St",
                city: "Another Town",
                state: "CA",
                zip: 12345,
                number: 123,
            },
            education: value_map!{
                high_school: "Anytown High School",
                college: "Anytown College",
            },
            height: 170,
        };

        value.merge(&local);

        assert_eq!(value.get("name").try_into_string().unwrap(), "David");
        assert_eq!(value.get("age").try_into_i64().unwrap(), 33);
        assert_eq!(value.get("height").try_into_i64().unwrap(), 170);
        assert_eq!(value.get("address").get("street").try_into_string().unwrap(), "123 Main St");
        assert_eq!(value.get("address").get("city").try_into_string().unwrap(), "Another Town");
        assert_eq!(value.get("address").get("state").try_into_string().unwrap(), "CA");
        assert_eq!(value.get("address").get("zip").try_into_i64().unwrap(), 12345);
        assert_eq!(value.get("address").get("number").try_into_i64().unwrap(), 123);
        assert_eq!(value.get("education").get("high_school").try_into_string().unwrap(), "Anytown High School");
        assert_eq!(value.get("education").get("college").try_into_string().unwrap(), "Anytown College");
    }

    #[test]
    fn test_clone_merge() {
        let value = value_map!{
            name: "David",
            age: 33,
            address: value_map!{
                street: "123 Main St",
                city: "Anytown",
                state: "CA",
                zip: 12345,
            },
        };

        let local = value_map!{
            address: value_map!{
                street: "123 Main St",
                city: "Another Town",
                state: "CA",
                zip: 12345,
                number: 123,
            },
            education: value_map!{
                high_school: "Anytown High School",
                college: "Anytown College",
            },
            height: 170,
        };

        let value = value.clone_merge(&local);

        assert_eq!(value.get("name").try_into_string().unwrap(), "David");
        assert_eq!(value.get("age").try_into_i64().unwrap(), 33);
        assert_eq!(value.get("height").try_into_i64().unwrap(), 170);
        assert_eq!(value.get("address").get("street").try_into_string().unwrap(), "123 Main St");
        assert_eq!(value.get("address").get("city").try_into_string().unwrap(), "Another Town");
        assert_eq!(value.get("address").get("state").try_into_string().unwrap(), "CA");
        assert_eq!(value.get("address").get("zip").try_into_i64().unwrap(), 12345);
        assert_eq!(value.get("address").get("number").try_into_i64().unwrap(), 123);
        assert_eq!(value.get("education").get("high_school").try_into_string().unwrap(), "Anytown High School");
        assert_eq!(value.get("education").get("college").try_into_string().unwrap(), "Anytown College");
    }
}