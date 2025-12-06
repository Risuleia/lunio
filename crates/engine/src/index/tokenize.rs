use unicode_normalization::UnicodeNormalization;

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .nfc()
        .collect::<String>()
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}