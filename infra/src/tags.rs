pub fn split_tags(tags: Vec<String>) -> Vec<String> {
    tags.iter()
        .flat_map(|x| x.split(','))
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

pub fn lowercase_separated_by_dash(tags: Vec<String>) -> Vec<String> {
    tags.iter()
        .map(|x| x.to_lowercase().replace(' ', "-"))
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}
#[cfg(test)]
pub mod splitting_tests {
    use super::*;

    // Note: Can override a function from production code to do some preprocessing
    // fn split_tags(tags: Vec<String>) -> Vec<String> {
    //     super::split_tags(tags.iter()).collect()
    // }
    #[test]
    fn keep_the_tags_as_is() {
        assert_eq!(
            vec!["hello".to_string(), "second".to_string()],
            split_tags(vec!["hello".to_string(), "second".to_string()])
        );
    }
    #[test]
    fn line_with_multiple_tags_separated_by_comma() {
        assert_eq!(
            vec![
                "hello".to_string(),
                "second".to_string(),
                "third".to_string(),
            ],
            split_tags(vec!["hello, second, third".to_string()])
        );
    }
    #[test]
    fn line_with_multiple_tags_separated_by_comma_and_spaces() {
        assert_eq!(
            vec![
                "hello".to_string(),
                "another".to_string(),
                "third".to_string(),
            ],
            split_tags(vec!["hello  , another, third".to_string(),])
        );
    }
    #[test]
    fn acceptance_test_multiple_lines_with_multiple_tags_separated_by_comma_and_spaces() {
        assert_eq!(
            vec![
                "hello".to_string(),
                "another".to_string(),
                "third".to_string(),
                "hello".to_string(),
                "another".to_string(),
                "third".to_string(),
            ],
            split_tags(vec![
                "hello  , another, third".to_string(),
                "hello  , another, third".to_string(),
            ])
        );
    }

    #[test]
    fn space_after_the_tag_is_deleted() {
        assert_eq!(
            vec!["hello".to_string(), "another".to_string(),],
            split_tags(vec!["hello, another       ".to_string(),])
        );
    }
    #[test]
    fn space_before_the_tag_is_deleted() {
        assert_eq!(
            vec!["hello".to_string(), "another".to_string(),],
            split_tags(vec!["    hello, another".to_string(),])
        );
    }
    #[test]
    fn split_tags_without_spaces() {
        assert_eq!(
            vec!["hello".to_string(), "another".to_string(),],
            split_tags(vec!["hello,another".to_string(),])
        );
    }
    #[test]
    fn empty_tag_gets_deleted() {
        assert_eq!(
            vec!["hello".to_string(),],
            split_tags(vec!["hello,  ,,      ".to_string(),])
        );
    }
}

#[cfg(test)]
pub mod lowercasing_tests {
    use super::*;

    #[test]
    fn lowercase_tags() {
        assert_eq!(
            vec!["hello".to_string(), "another".to_string(),],
            lowercase_separated_by_dash(vec!["hello".to_string(), "another".to_string(),])
        );
    }

    #[test]
    fn lowercase_tags_should_be_idempotent() {
        assert_eq!(
            vec!["hello".to_string(), "another".to_string(),],
            lowercase_separated_by_dash(vec!["Hello".to_string(), "Another".to_string(),])
        );
    }
    #[test]
    fn separate_by_dash() {
        assert_eq!(
            vec!["hello-world".to_string(), "another-tag".to_string(),],
            lowercase_separated_by_dash(
                vec!["Hello World".to_string(), "Another Tag".to_string(),]
            )
        );
    }
}
