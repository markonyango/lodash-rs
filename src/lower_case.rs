pub fn lower_case(string: &str) -> String {
  string.to_lowercase()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_german_umlaute() {
    assert_eq!(lower_case("Ä"), "ä");
    assert_eq!(lower_case("Ö"), "ö");
    assert_eq!(lower_case("Ü"), "ü");
    assert_eq!(lower_case("SS"), "ss");
  }
  
  #[test]
  fn works_with_snake_case() {
    assert_eq!(lower_case("Test_String"), "test_string");
  }

  #[test]
  fn works_with_camel_case() {
    assert_eq!(lower_case("TestString"), "teststring");
  }

  #[test]
  fn works_with_spaces() {
    assert_eq!(lower_case("Test String"), "test string");
  
  }
  #[test]
  fn works_with_digits_in_between() {
    assert_eq!(lower_case("Test123String"), "test123string");
  }
}
