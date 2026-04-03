use crate::case::class::to_class_case;

/// Demodulize a `&str`
///
/// ```
/// use cruet::string::demodulize::demodulize;
/// let mock_string: &str = "Bar";
/// let expected_string: String = "Bar".to_owned();
/// let asserted_string: String = demodulize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
/// ```
/// use cruet::string::demodulize::demodulize;
/// let mock_string: &str = "::Bar";
/// let expected_string: String = "Bar".to_owned();
/// let asserted_string: String = demodulize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
/// ```
/// use cruet::string::demodulize::demodulize;
/// let mock_string: &str = "Foo::Bar";
/// let expected_string: String = "Bar".to_owned();
/// let asserted_string: String = demodulize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
/// ```
/// use cruet::string::demodulize::demodulize;
/// let mock_string: &str = "Test::Foo::Bar";
/// let expected_string: String = "Bar".to_owned();
/// let asserted_string: String = demodulize(mock_string);
/// assert!(asserted_string == expected_string);
/// ```
pub fn demodulize(non_demodulize_string: &str) -> String {
    match non_demodulize_string.rsplit_once("::") {
        Some((_, last)) => to_class_case(last),
        None => non_demodulize_string.to_owned(),
    }
}
