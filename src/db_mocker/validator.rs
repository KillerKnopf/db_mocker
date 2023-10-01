use anyhow::Result;

mod key_word_checker;

pub fn validate_name(name: &str) -> bool {
    if key_word_checker::check_for_sql_keywords(name.to_uppercase().as_str()) {
        return false;
    }

    // Starting with number, camel case instead of snake case

    true
}
