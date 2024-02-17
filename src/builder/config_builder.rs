use std::collections::HashMap;

use crate::{Value, ConfigError};
use crate::traits::{PriorityHandler, ConfigReader};

#[derive(Debug, PartialEq, Clone)]
pub struct ConfigBuilder {
    pub(crate) priority_list: Vec<u64>,
    pub(crate) priority_first: u64,
    pub(crate) priority_last: u64,
    pub(crate) values: HashMap<u64, Value>,
}

impl ConfigBuilder {
    fn set_value(&mut self, priority: u64, value: Value) {
        if priority == 0 {
            panic!("Priority 0 is reserved for the base value");
        }

        if self.priority_list.contains(&priority) {
            panic!("Priority {} is already in use", priority);
        }

        if !value.is_map() {
            panic!("Value is not a map");
        }

        if self.priority_first == 0 || priority < self.priority_first {
            self.priority_first = priority;
        }

        if priority > self.priority_last {
            self.priority_last = priority;
        }

        self.priority_list.push(priority);
        self.values.insert(priority, value);
    }

    fn validate_priority(&self, priority: Option<u64>) -> Result<u64, ConfigError> {
        match priority {
            Some(priority) => {
                if priority == 0 {
                    return Err(ConfigError::BuilderPriorityReserved);
                }

                if self.priority_list.contains(&priority) {
                    return Err(ConfigError::BuilderPriorityInUse(priority));
                }

                Ok(priority)
            },
            None => {
                Err(ConfigError::BuilderPriorityNotFound)
            }
        }
    }

    /** Reads the configuration source and returns the priority used. */
    pub fn add_config<T>(&mut self, source: &T) -> Result<u64, ConfigError> where T: ConfigReader {
        let priority = self.validate_priority(source.get_priority(self))?;
        let value = source.read()?;

        if !value.is_map() {
            return Err(ConfigError::NotMapValue);
        }

        self.set_value(priority, value);
        
        Ok(priority)
    }

    /** Changes where it starts to look for a free slot, defaults to 100 */
    pub fn start_from(&mut self, priority: u64) -> Result<(), ConfigError> {
        if !self.priority_list.contains(&priority) {
            match self.after(&priority) {
                Some(next) => {
                    self.priority_last = next - 1;
                },
                None => {
                    return Err(ConfigError::BuilderPriorityNotFound);
                }
            }
        }

        self.priority_last = priority;
        Ok(())
    }

    /** Merge all values and return the result */
    pub fn build(&self) -> Value {
        let mut base = self.values.get(&0)
            .expect("Base value is not defined")
            .clone();
        
        let mut sorted_list = self.priority_list.clone();
        sorted_list.sort();

        for priority in sorted_list {
            if priority == 0 {
                continue;
            }

            let value = self.values
                .get(&priority)
                .expect(format!("Value for priority {} is not defined", priority).as_str());

            base.merge(value);
        }

        base
    }

    pub fn new() -> Self {
        let base_value = Value::new_map();
        let mut values = HashMap::new();
        values.insert(0, base_value);

        Self {
            priority_list: Vec::new(),
            priority_first: 0,
            priority_last: 100,
            values,
        }
    }

    pub fn from_base(base: Value) -> Result<Self, ConfigError> {
        if !base.is_map() {
            return Err(ConfigError::NotMapValue);
        }

        let mut values = HashMap::new();
        values.insert(0, base);

        Ok(Self {
            priority_list: Vec::new(),
            priority_first: 0,
            priority_last: 100,
            values,
        })
    }
}

impl PriorityHandler for ConfigBuilder {
    fn is_free(&self, index: &u64) -> bool {
        if index == &0 {
            return false;
        }

        !self.priority_list.contains(index)
    }

    fn next(&self) -> Option<u64> {
        if self.priority_last == u64::MAX {
            return None;
        }

        Some(self.priority_last + 1)
    }

    fn after(&self, index: &u64) -> Option<u64> {
        if index == &u64::MAX {
            return None;
        }

        let mut next = index + 1;
        while !self.is_free(&next) {
            if next == u64::MAX {
                return None;
            }
            next += 1;
        }
        Some(next)
    }

    fn before(&self, index: &u64) -> Option<u64> {
        if index == &u64::MIN {
            return None;
        }

        let mut next = index - 1;
        while !self.is_free(&next) {
            if next == u64::MIN {
                return None;
            }
            next -= 1;
        }
        Some(next)
    }

    fn last_used(&self) -> u64 {
        return self.priority_last;
    }

    fn first_used(&self) -> u64 {
        return self.priority_first;
    }

    fn used(&self) -> &Vec<u64> {
        return &self.priority_list;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::builder::Priority;

    #[test]
    fn test_priority_handler() {
        let mut builder = ConfigBuilder {
            priority_list: Vec::new(),
            priority_first: 0,
            priority_last: 0,
            values: HashMap::new(),
        };

        assert_eq!(builder.is_free(&0), false);
        assert_eq!(builder.is_free(&1), true);

        assert_eq!(builder.next(), Some(1));
        assert_eq!(builder.after(&10), Some(11));
        assert_eq!(builder.before(&10), Some(9));
        assert_eq!(builder.after(&u64::MAX), None);
        assert_eq!(builder.before(&u64::MIN), None);

        builder.priority_list.push(10);
        builder.priority_last = 10;

        assert_eq!(builder.is_free(&10), false);
        assert_eq!(builder.is_free(&1), true);
        assert_eq!(builder.next(), Some(11));
        assert_eq!(builder.after(&9), Some(11));
        assert_eq!(builder.before(&11), Some(9));
    }

    #[test]
    fn test_priority_enum() {
        let mut builder = ConfigBuilder {
            priority_list: Vec::new(),
            priority_first: 0,
            priority_last: 0,
            values: HashMap::new(),
        };

        assert_eq!(Priority::Any.get_priority(&builder), Some(1));
        assert_eq!(Priority::After(10).get_priority(&builder), Some(11));
        assert_eq!(Priority::Before(10).get_priority(&builder), Some(9));
        assert_eq!(Priority::First.get_priority(&builder), Some(1));
        assert_eq!(Priority::FirstAvailable.get_priority(&builder), Some(1));
        assert_eq!(Priority::Last.get_priority(&builder), Some(u64::MAX));
        assert_eq!(Priority::LastAvailable.get_priority(&builder), Some(u64::MAX));

        builder.priority_list.push(10);
        builder.priority_last = 10;

        assert_eq!(Priority::Any.get_priority(&builder), Some(11));
        assert_eq!(Priority::After(9).get_priority(&builder), Some(11));
        assert_eq!(Priority::Before(11).get_priority(&builder), Some(9));

        builder.priority_list.push(1);
        builder.priority_list.push(u64::MAX);
        builder.priority_first = 1;
        builder.priority_last = u64::MAX;

        assert_eq!(Priority::Any.get_priority(&builder), Some(11));
        assert_eq!(Priority::After(1).get_priority(&builder), Some(2));
        assert_eq!(Priority::Before(u64::MAX).get_priority(&builder), Some(u64::MAX-1));
        assert_eq!(Priority::First.get_priority(&builder), None);
        assert_eq!(Priority::FirstAvailable.get_priority(&builder), Some(2));
        assert_eq!(Priority::Last.get_priority(&builder), None);
        assert_eq!(Priority::LastAvailable.get_priority(&builder), Some(u64::MAX-1));
    }
}