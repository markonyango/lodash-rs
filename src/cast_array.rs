
/**
Casts value as an array.
If it is a n array already it will be nested.

Example
````
use lodash_rs::cast_array;

let res = cast_array(0);
println!("{:?}", res); // [0]

let res = cast_array([0]);
println!("{:?}", res); // [[0]]
*/
pub fn cast_array<T>(value: T) -> Vec<T> {
    vec![value]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_strings() {
        assert_eq!(cast_array("Word"), ["Word"]);
    }

    #[test]
    fn works_with_numbers() {
        assert_eq!(cast_array(0), [0]);
    }
}
