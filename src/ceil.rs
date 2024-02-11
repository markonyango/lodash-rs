pub fn ceil(value: f64, precision: Option<i32>) -> f64 {
  match precision {
    Some(p) => {
      (value * (10.0_f64.powi(p))).ceil() / 10.0_f64.powi(p)
    },
    None => value.ceil()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works() {
    assert_eq!(ceil(3.49, Some(1)), 3.5);
    assert_eq!(ceil(-2.0, Some(1)), -2.0);
    assert_eq!(ceil(4.006, None), 5.0);
    assert_eq!(ceil(6.004, Some(2)), 6.01);
    assert_eq!(ceil(6040., Some(-2)), 6100.);
  }
}
