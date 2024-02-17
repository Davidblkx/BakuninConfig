use crate::traits::PriorityHandler;

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    /** Must be the first to be merged, or none */
    First,
    /** Tries to add at the first available slot */
    FirstAvailable,
    /** Must be the last to be merged, or none */
    Last,
    /** Tries to add at the last available slot */
    LastAvailable,
    /** Must be merged before the given priority, or none */
    Before(u64),
    /** Must be merged after the given priority, or none */
    After(u64),
    /** 
     * Tries to find any available priority slot.
     * Defaults to the slot after the last used.
     * If the last used is MAX, defaults to the next available slot:
        * if you have \[1, 2, 3, MAX\] returns 4
        * if you have \[1, 2, 3, 50, MAX\] returns 51
        * if you have \[MAX\] returns 1
     */
    Any,
}

impl Priority {
    pub fn get_priority<T>(&self, handler: &T) -> Option<u64> where T: PriorityHandler {
        match self {
            Priority::First => {
                if handler.is_free(&1) {
                    Some(1)
                } else {
                    None
                }
            },
            Priority::FirstAvailable => handler.after(&0),
            Priority::Last => {
                if handler.is_free(&u64::MAX) {
                    Some(u64::MAX)
                } else {
                    None
                }
            },
            Priority::LastAvailable => {
                if handler.is_free(&u64::MAX) {
                    Some(u64::MAX)
                } else {
                    handler.before(&u64::MAX)
                }
            }
            Priority::Before(index) => handler.before(index),
            Priority::After(index) => handler.after(index),
            Priority::Any => match handler.next() {
                Some(index) => Some(index),
                None => {
                    if handler.used().len() <= 1 {
                        return Some(1);
                    }

                    let mut used = handler.used().clone();
                    used.sort();

                    match used.get(used.len() - 2) {
                        Some(index) => match handler.after(index) {
                            Some(index) => Some(index),
                            None => handler.before(&u64::MAX)
                        }
                        None => Some(1) // Should never happen
                    }
                }
            },
        }
    }
}