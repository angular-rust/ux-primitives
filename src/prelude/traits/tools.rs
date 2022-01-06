use super::{Encrypter, Priority};

/// The Tools should be implemented by objects intended to provide tool box functionality.
/// 
/// Many of these tools are need in the minimalist implementation of the interfaces.
/// The other functions are often regularly used in entity creation.
///
pub trait Tools: Encrypter {
    // /// Used as an arbitrarily high number to avoid infinity and division by zero issues.
    // fn big_number(&self) -> i32;

    /// Creates a Globally Unique Identifier.
    /// 
    /// # Arguments
    ///
    /// * `isSmall` - If true returns an 8 bit identifier, 16 bit otherwise. (optional, default: false)
    /// * `prefix` - Adds characters to the front of the GUID. (optional, default: empty)
    ///
    /// Return: A Globally Unique Identifier.
    /// 
    fn create_guid(&self, is_small: Option<bool>, prefix: Option<String>) -> String;

    /// Find the tween of two values.
    /// 
    /// # Arguments
    ///
    /// * `originalValue` - Value A.
    /// * `newValue` - Value B.
    /// * `ease` - The proportion of A:B.
    ///
    /// Return: The proportional value of A:B.
    /// 
    fn ease(&self, original_value: f32, new_value: f32, ease: f32) -> f32;

    /// Sorting functio'n for String collections.
    /// 
    /// # Arguments
    ///
    /// * `a` - Value A.
    /// * `b` - Value B.
    ///
    /// Return: -1 if A<B, 1 if A>B or 0 if A==B.
    /// 
    fn sort_by_string(&self, a: String, b: String) -> i32;

    /// Sorting functio'n for Int collections.
    /// 
    /// # Arguments
    ///
    /// * `a` - Value A.
    /// * `b` - Value B.
    ///
    /// Return: -1 if A<B, 1 if A>B or 0 if A==B.
    /// 
    fn sort_by_int(&self, a: i32, b: i32) -> i32;

    /// Sorting functio'n for IPriority collections.
    /// 
    /// # Arguments
    ///
    /// * `a` - Value A.
    /// * `b` - Value B.
    ///
    /// Return: -1 if A<B, 1 if A>B or 0 if A==B.
    /// 
    fn sort_by_priority(&self, a: Box<dyn Priority>, b: Box<dyn Priority>) -> i32;

    /// Creates a copy of a string with the first character uppercased.
    /// 
    /// # Arguments
    ///
    /// * `value` - The string to transform.
    ///
    /// Return: Copy of a string with the first character uppercased.
    /// 
    fn to_upper_case_first(&self, value: String) -> String;

    /// Turns a word or sentence into camelCase.
    /// E.g. "this example string" becomes "thisExampleString".
    /// 
    /// # Arguments
    ///
    /// * `value` - The string to transform.
    /// * `isUpper` - If true returns PascalCase (first character uppercased). (optional, default: false)
    ///
    /// Return: camelCase or PascalCase representation of a string.
    /// 
    fn to_camel_case(&self, value: String, is_upper: Option<bool>) -> String;

    /// Turns a word of sentence into CONST_CASE.
    /// E.g. "this example string" becomes "THIS_EXAMPLE_STRING".
    /// 
    /// # Arguments
    ///
    /// * `value` - The string to transform.
    ///
    /// Return: CONST_CASE representation of a string.
    /// 
    fn to_const_case(&self, value: String) -> String;

    /// Reverts a camelCase string to a word or phrase.
    /// E.g. "thisExampleString" becomes "this example string".
    /// 
    /// # Arguments
    ///
    /// * `value` - The camelCase string to revert.
    ///
    /// Return: Word or phrase.
    /// 
    fn from_camel_case(&self, value: String) -> String;

    /// Reverts a CONST_CASE string to a word or phrase.
    /// E.g. "THIS_EXAMPLE_STRING" becomes "this example string"
    /// 
    /// # Arguments
    ///
    /// * `value` - The CONST_CASE string to revert.
    ///
    /// Return: Word or phrase.
    /// 
    fn from_const_case(&self, value: String) -> String;

    /// Reverts either a camelCase or CONST_CASE string to a word or phrase.
    /// 
    /// # Arguments
    ///
    /// * `value` - The camelCase or CONST_CASE string.
    ///
    /// Return: Word or phrase.
    /// 
    fn to_words(&self, value: String) -> String;

    /// Clamps a value between a floor and ceiling boundary.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to clamp.
    /// * `min` - The floor.
    /// * `max` - The ceiling.
    ///
    /// Return: Value >= floor and <= ceiling.
    /// 
    fn limit(&self, value: f32, min: f32, max: f32) -> f32;

    /// Wraps a value between a floor and ceiling boundary.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to wrap.
    /// * `min` - The floor.
    /// * `max` - The ceiling.
    ///
    /// Return: A value between floor and ceiling proportional to over or under shoot.
    /// 
    fn range(&self, value: f32, min: f32, max: f32) -> f32;

    // /// Replaces two objects with the content of the other.
    // fn swap<T>(&self, a: T, b: T);

    /// Not divisible by two.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// Return: True if value not divisible by two.
    /// 
    fn is_odd(&self, value: i32) -> bool;

    /// Divisible by two.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// Return: True if value divisible by two.
    /// 
    fn is_even(&self, value: i32) -> bool;

    /// Determine whether a value is less than zero, equal to zero or greater than zero.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// Return: -1 if <0, 1 if >0, 0 otherwise.
    /// 
    fn sgn(&self, value: f32) -> i32;

    // /// Determine whether a value is true.
    // /// Results vary based on the context of checked value.  Usually safer to do your own Bool checks.
    // /// * `value` - The value to check.
    // /// @return	True if the value is true.
    // fn is_bool(&self, value: T) -> bool;

    /// Calculate the nearest square number to a given value.
    /// Useful for performance routines.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to check.
    ///
    /// Return: A square number nearest to the value.
    /// 
    fn nearest_square(&self, value: f32) -> i32;

    /// Calculates the distance between two coordinates.
    /// 
    /// # Arguments
    ///
    /// * `startX` - The starting position horizontal coordinate.
    /// * `startY` - The starting position vertical coordinate.
    /// * `endX` - The ending position horizontal coordinate.
    /// * `endY` - The ending position vertical coordinate.
    /// * `isSquared` - For performance.  Set this to true square comparator (to avoid sqrt). (optional, default: false)
    ///
    /// Return: The distance between two coordinates.
    /// 
    fn distance(&self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, is_squared: Option<bool>) -> f32;

    /// Creates a string representing a clock in the format "hh'mm'ss".
    /// Uses IFactory.targetFramerate to determine the duration from updates.
    /// 
    /// # Arguments
    ///
    /// * `updates` - The update cycles elapsed in the duration.
    /// * `delimiter` - The character used to separate the components (default: "'").
    ///
    /// Return: String representing a clock in the format "hh:mm:ss".
    /// 
    fn convert_updates_to_formatted_time(&self, updates: i32, delimiter: Option<String>) -> String;

    /// Creates a string representing a clock in the format "hh'mm'ss".
    /// 
    /// # Arguments
    ///
    /// * `age` - The time elapsed in the duration as milliseconds.
    /// * `delimiter` - The character used to separate the components (default: "'").
    ///
    /// Return: String representing a clock in the format "hh:mm:ss".
    /// 
    fn convert_age_to_formatted_time(&self, age: i32, delimiter: Option<String>) -> String;

    // /// Randomly sorts an array.
    // fn shuffle<T>(&self, array: Vec<T>) -> Vec<T>;

    // /// Creates any enumerator from the supplied class.
    // fn get_random_type<T>(&self, e: Enum<T>) -> T;

    /// Converts an Int to a Hex string.
    /// 
    /// # Arguments
    ///
    /// * `value` - The Int to convert.
    ///
    /// Return: Hex value.
    /// 
    fn int_to_hex(&self, value: i32) -> String;

    // /// Converts an object into a serialized string.
    // /// * `value` - The object to convert.
    // /// @return	The serialized object.
    // fn serialize(&self, value: T) -> String;

    // /// Restores an object from a serialized string.
    // /// * `value` - The serialised object.
    // /// @return	The object to restore.
    // fn unserialize(&self, value: String) -> T;
}
