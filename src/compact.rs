use std::error::Error;

pub trait Compactable<T> {
  fn is_compactable(&self) -> bool;
}

impl Compactable<u32> for u32 {
  fn is_compactable(&self) -> bool {
    return *self != 0
  }
}

impl Compactable<i32> for i32 {
  fn is_compactable(&self) -> bool {
    *self != 0
  }
}

impl Compactable<&str> for &str {
  fn is_compactable(&self) -> bool {
    !self.is_empty()
  }
}

impl Compactable<bool> for bool {
  fn is_compactable(&self) -> bool {
    *self
  }
}

impl<T> Compactable<Option<T>> for Option<T> {
  fn is_compactable(&self) -> bool {
    self.is_some()
  }
}

impl<T,E> Compactable<Result<T,E>> for Result<T,E>
where T: Sized, E: Error + Sized {
  fn is_compactable(&self) -> bool {
    self.is_ok()
  }
}

pub fn compact<T, I>(value: I) -> Vec<T>
where I: IntoIterator<Item = T>, T: Compactable<T> {
  value.into_iter()
    .filter(|x| x.is_compactable())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works() {
    assert_eq!(compact(vec![0,1,2,3]), vec![1,2,3]);
    assert_eq!(compact(vec!["world","","hello"]), vec!["world","hello"]);
    assert_eq!(compact(vec![Some("world"),None,Some("hello")]), vec![Some("world"),Some("hello")]);
    
    let filtered = compact(vec![Ok("world"),Err(std::io::Error::new(std::io::ErrorKind::Other, "failed!")),Ok("hello")]);
    assert!(filtered.iter().all(Result::is_ok));
  }
}
