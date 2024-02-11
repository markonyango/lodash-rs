pub fn upper_case(string: &str) -> String {
  string.to_uppercase()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_german_umlaute() {
    assert_eq!(upper_case("ä"), "Ä");
    assert_eq!(upper_case("ö"), "Ö");
    assert_eq!(upper_case("ü"), "Ü");
    assert_eq!(upper_case("ß"), "SS");
  }
  
  #[test]
  fn works_with_snake_case() {
    assert_eq!(upper_case("Test_String"), "TEST_STRING");
  }

  #[test]
  fn works_with_camel_case() {
    assert_eq!(upper_case("TestString"), "TESTSTRING");
  }

  #[test]
  fn works_with_spaces() {
    assert_eq!(upper_case("Test String"), "TEST STRING");
  
  }
  #[test]
  fn works_with_digits_in_between() {
    assert_eq!(upper_case("Test123String"), "TEST123STRING");
  }
}
