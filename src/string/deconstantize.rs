use crate::case::class::to_class_case;

/// Deconstantizes a `&str`
///
/// ```
/// use cruet::string::deconstantize::deconstantize;
/// let mock_string: &str = "Bar";
/// let expected_string: String = "".to_owned();
/// let asserted_string: String = deconstantize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
/// ```
/// use cruet::string::deconstantize::deconstantize;
/// let mock_string: &str = "::Bar";
/// let expected_string: String = "".to_owned();
/// let asserted_string: String = deconstantize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
/// ```
/// use cruet::string::deconstantize::deconstantize;
/// let mock_string: &str = "Foo::Bar";
/// let expected_string: String = "Foo".to_owned();
/// let asserted_string: String = deconstantize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
/// ```
/// use cruet::string::deconstantize::deconstantize;
/// let mock_string: &str = "Test::Foo::Bar";
/// let expected_string: String = "Foo".to_owned();
/// let asserted_string: String = deconstantize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
pub fn deconstantize(non_deconstantized_string: &str) -> String {
    match non_deconstantized_string.rsplit_once("::") {
        Some((prefix, _)) => {
            if prefix.is_empty() {
                "".to_owned()
            } else {
                match prefix.rsplit_once("::") {
                    Some((_, second_last)) => to_class_case(second_last),
                    None => to_class_case(prefix),
                }
            }
        }
        None => "".to_owned(),
    }
}
