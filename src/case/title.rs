use crate::case::*;
/// Converts a `&str` to `Title Case` `String`
///
/// ```
/// use cruet::case::title::to_title_case;
///
/// assert!(to_title_case("Foo bar") == "Foo Bar");
/// assert!(to_title_case("FooBar") == "Foo Bar");
/// assert!(to_title_case("fooBar") == "Foo Bar");
/// assert!(to_title_case("FOO_BAR") == "Foo Bar");
/// assert!(to_title_case("foo_bar") == "Foo Bar");
/// assert!(to_title_case("foo-bar") == "Foo Bar");
/// ```
pub fn to_title_case(non_title_case_string: &str) -> String {
    let options = CamelOptions {
        new_word: true,
        last_char: ' ',
        first_word: true,
        injectable_char: ' ',
        has_separator: true,
        inverted: false,
        concat_num: false,
    };
    to_case_camel_like(non_title_case_string, options)
}

/// Determines if a `&str` is `Title Case`
///
/// ```
/// use cruet::case::title::is_title_case;
///
/// assert!(is_title_case("Foo Bar String That Is Really Really Long"));
/// assert!(!is_title_case("foo-bar-string-that-is-really-really-long"));
/// assert!(!is_title_case("FooBarIsAReallyReallyLongString"));
/// assert!(!is_title_case("fooBarIsAReallyReallyLongString"));
/// assert!(!is_title_case("FOO_BAR_STRING_THAT_IS_REALLY_REALLY_LONG"));
/// assert!(!is_title_case("foo_bar_string_that_is_really_really_long"));
/// assert!(!is_title_case("Foo bar string that is really really long"));
/// assert!(!is_title_case("foo"));
/// ```
pub fn is_title_case(test_string: &str) -> bool {
    test_string == to_title_case(test_string)
}

#[cfg(test)]
mod tests {
    use super::{is_title_case, to_title_case};

    #[test]
    fn from_camel_case() {
        let convertible_string: String = "fooBar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_pascal_case() {
        let convertible_string: String = "FooBar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_kebab_case() {
        let convertible_string: String = "foo-bar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_sentence_case() {
        let convertible_string: String = "Foo bar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_title_case() {
        let convertible_string: String = "Foo Bar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_train_case() {
        let convertible_string: String = "Foo-Bar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_screaming_snake_case() {
        let convertible_string: String = "FOO_BAR".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_snake_case() {
        let convertible_string: String = "foo_bar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn from_case_with_loads_of_space() {
        let convertible_string: String = "foo           bar".to_owned();
        let expected: String = "Foo Bar".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn a_name_with_a_dot() {
        let convertible_string: String = "Robert C. Martin".to_owned();
        let expected: String = "Robert C Martin".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn random_text_with_bad_chars() {
        let convertible_string: String = "Random text with *(bad) chars".to_owned();
        let expected: String = "Random Text With Bad Chars".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn trailing_bad_chars() {
        let convertible_string: String = "trailing bad_chars*(()())".to_owned();
        let expected: String = "Trailing Bad Chars".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn leading_bad_chars() {
        let convertible_string: String = "-!#$%leading bad chars".to_owned();
        let expected: String = "Leading Bad Chars".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn wrapped_in_bad_chars() {
        let convertible_string: String =
            "-!#$%wrapped in bad chars&*^*&(&*^&(<><?>><?><>))".to_owned();
        let expected: String = "Wrapped In Bad Chars".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn has_a_sign() {
        let convertible_string: String = "has a + sign".to_owned();
        let expected: String = "Has A Sign".to_owned();
        assert_eq!(to_title_case(&convertible_string), expected)
    }

    #[test]
    fn is_correct_from_camel_case() {
        let convertible_string: String = "fooBar".to_owned();
        assert_eq!(is_title_case(&convertible_string), false)
    }

    #[test]
    fn is_correct_from_pascal_case() {
        let convertible_string: String = "FooBar".to_owned();
        assert_eq!(is_title_case(&convertible_string), false)
    }

    #[test]
    fn is_correct_from_kebab_case() {
        let convertible_string: String = "foo-bar".to_owned();
        assert_eq!(is_title_case(&convertible_string), false)
    }

    #[test]
    fn is_correct_from_sentence_case() {
        let convertible_string: String = "Foo bar".to_owned();
        assert_eq!(is_title_case(&convertible_string), false)
    }

    #[test]
    fn is_correct_from_title_case() {
        let convertible_string: String = "Foo Bar".to_owned();
        assert_eq!(is_title_case(&convertible_string), true)
    }

    #[test]
    fn is_correct_from_train_case() {
        let convertible_string: String = "Foo-Bar".to_owned();
        assert_eq!(is_title_case(&convertible_string), false)
    }

    #[test]
    fn is_correct_from_screaming_snake_case() {
        let convertible_string: String = "FOO_BAR".to_owned();
        assert_eq!(is_title_case(&convertible_string), false)
    }

    #[test]
    fn is_correct_from_snake_case() {
        let convertible_string: String = "foo_bar".to_owned();
        assert_eq!(is_title_case(&convertible_string), false)
    }
}
