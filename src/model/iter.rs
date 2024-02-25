use super::Value;

pub struct ValueIter<'a> {
    value: &'a Value,
    index: usize,
}

impl<'a> ValueIter<'a> {
    pub fn new(value: &'a Value) -> Self {
        Self { value, index: 0 }
    }
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            Value::Array(array) => {
                let value = array.get(self.index);
                self.index += 1;
                value
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value_vec;

    use super::*;

    #[test]
    fn test_value_iter() {
        let value = value_vec!(1, 2, 3);
        let mut iter = value.iter();

        assert_eq!(Value::Integer(1), *iter.next().unwrap());
        assert_eq!(Value::Integer(2), *iter.next().unwrap());
        assert_eq!(Value::Integer(3), *iter.next().unwrap());
    }
}