pub fn capitalize(string: &str) -> String {
  if string.len() < 1 {
    return String::from("");
  }

  format!("{}{}", &string[0..1].to_uppercase(), &string[1..].to_lowercase())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_single_word() {
    assert_eq!(capitalize("WORD"), "Word");
  }

  #[test]
  fn works_with_multiple_words() {
    assert_eq!(capitalize("WORD WORD"), "Word word");
  }

  #[test]
  fn works_with_zero_length_string() {
    assert_eq!(capitalize(""), "");
  }

  #[test]
  fn works_with_single_character() {
    assert_eq!(capitalize("a"), "A");
  }
}
