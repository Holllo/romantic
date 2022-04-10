use romantic::Roman;

use test_case::test_case;

#[test_case(00, ""; "empty")]
#[test_case(01, "A"; "one")]
#[test_case(02, "AA"; "two")]
#[test_case(03, "AAA"; "three")]
#[test_case(04, "AB"; "four")]
#[test_case(05, "B"; "five")]
#[test_case(06, "BA"; "six")]
#[test_case(07, "BAA"; "seven")]
#[test_case(08, "BAAA"; "eight")]
#[test_case(09, "AC"; "nine")]
#[test_case(10, "C"; "ten")]
fn test_to_string<T: num::PrimInt + num::Signed + ToString>(
  input: T,
  expected: &str,
) {
  let custom = Roman::new(&['A', 'B', 'C']);
  assert_eq!(custom.to_string(input).unwrap(), expected);
}

#[test_case("", 00; "empty")]
#[test_case("A", 01; "one")]
#[test_case("AA", 02; "two")]
#[test_case("AAA", 03; "three")]
#[test_case("AB", 04; "four")]
#[test_case("B", 05; "five")]
#[test_case("BA", 06; "six")]
#[test_case("BAA", 07; "seven")]
#[test_case("BAAA", 08; "eight")]
#[test_case("AC", 09; "nine")]
#[test_case("C", 10; "ten")]
fn test_from_str(input: &str, expected: i32) {
  let custom = Roman::new(&['A', 'B', 'C', 'D']);
  assert_eq!(custom.from_str::<i32>(input).unwrap(), expected);
}
