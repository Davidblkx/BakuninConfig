use crate::{ConfigError, Value};

pub trait PriorityHandler {
    /** Checks if a priority is available */
    fn is_free(&self, index: &u64) -> bool;
    /** 
     * Returns the next available priority 
     * If the u64::MAX is reached, None is returned.
     */
    fn next(&self) -> Option<u64>;
    /** 
     * Returns the next available priority after the given one.
     * If the u64::MAX is reached, None is returned.
     */
    fn after(&self, index: &u64) -> Option<u64>;
    /** 
     * Returns the next available priority before the given one.
     * If the u64::MIN is reached, None is returned.
     */
    fn before(&self, index: &u64) -> Option<u64>;

    /** Returns the last used priority */
    fn last_used(&self) -> u64;
    /** Returns the first used priority */
    fn first_used(&self) -> u64;
    /** Returns the list of used priorities */
    fn used(&self) -> &Vec<u64>;
}

/**
 * The ConfigReader trait is used to read values from a configuration source.
 * Result value must be a Value::Map.
 */
pub trait ConfigReader {
    /** Name used in logs */
    fn name(&self) -> String;
    /** Read the configuration source and return a Value::Map */
    fn read(&self) -> Result<Value, ConfigError>;
    /** 
     * Calculates the priority that should be used when building the final value.
     * 0 is reserved for the base value.
     * 1 is the first to be loaded.
     * If priority is in use, the next available priority should will be used.
     * If return value is None, the value will not be used.
     */
    fn get_priority<T>(&self, handler: &T) -> Option<u64> where T: PriorityHandler;
}