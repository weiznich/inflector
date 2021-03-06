/// Converts a `String` to lowercase `String`
///
/// #Examples
/// ```
/// use inflector::cases::lowercase::to_lower_case;
///
///
/// // upcase_FoObAR_as_foobar() {
///     let mock_string: String = "FoObAR".to_string();
///     let expected_string: String = "foobar".to_string();
///     let asserted_string: String = to_lower_case(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn to_lower_case(non_lower_string : String) -> String {
    // See https://github.com/calebmer/inflections/blob/master/src/case.rs#L37 for where this
    // implementation comes from.
    non_lower_string
        .chars()
        .flat_map(char::to_lowercase)
        .collect()
}

/// Determines if a `String` is lowercase
///
/// #Examples
/// ```
/// use inflector::cases::lowercase::is_lower_case;
///
///
/// // returns_truthy_value_for_is_lower_case_when_lowercase() {
///     let mock_string: String = "foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_lower_case(mock_string);
///     assert!(asserted_bool == true);
///
/// ```
/// ```
/// use inflector::cases::lowercase::is_lower_case;
///
///
/// // returns_falsey_value_for_is_lower_case_when_Startcased() {
///     let mock_string: String = "Foobarisareallyreallylongstring".to_string();
///     let asserted_bool: bool = is_lower_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
/// ```
/// use inflector::cases::lowercase::is_lower_case;
///
///
/// // returns_falsey_value_for_is_lower_case_when_uppercase() {
///     let mock_string: String = "FOOBARSTRINGTHATISREALLYREALLYLONG".to_string();
///     let asserted_bool: bool = is_lower_case(mock_string);
///     assert!(asserted_bool == false);
///
/// ```
pub fn is_lower_case(test_string: String) -> bool{
    test_string == to_lower_case(test_string.clone())
}
