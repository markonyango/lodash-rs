pub fn chunk<T: Clone>(value: Vec<T>, size: usize) -> Vec<Vec<T>> {
  if size < 1 {
    return value.chunks(1).map(|c| c.into()).collect()
  }

  value.chunks(size).map(|c| c.into()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works() {
    assert_eq!(chunk(vec!["a","b","c","d"], 0), vec![vec!["a"], vec!["b"], vec!["c"], vec!["d"]]);
    assert_eq!(chunk(vec!["a","b","c","d"], 1), vec![vec!["a"], vec!["b"], vec!["c"], vec!["d"]]);
    assert_eq!(chunk(vec!["a","b","c","d"], 2), vec![vec!["a", "b"], vec!["c", "d"]]);
    assert_eq!(chunk(vec!["a","b","c","d"], 3), vec![vec!["a", "b", "c"], vec!["d"]]);
  }
}
