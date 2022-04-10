use romantic::Roman;

use test_case::test_case;

#[test_case("A"; "invalid character")]
fn test_from_str_error(input: &str) {
  assert!(Roman::default().from_str::<i32>(input).is_err());
}

#[test_case(4000; "too high")]
#[test_case(-100; "negative")]
fn test_to_string_error(input: i32) {
  assert!(Roman::default().to_string(input).is_err());
}
