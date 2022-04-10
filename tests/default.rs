use romantic::Roman;

use test_case::test_case;

#[test_case(0, ""; "empty")]
#[test_case(3888, "MMMDCCCLXXXVIII"; "all characters")]
#[test_case(3999, "MMMCMXCIX"; "maximum")]
#[test_case(01, "I"; "one")]
#[test_case(02, "II"; "two")]
#[test_case(03, "III"; "three")]
#[test_case(04, "IV"; "four")]
#[test_case(05, "V"; "five")]
#[test_case(06, "VI"; "six")]
#[test_case(07, "VII"; "seven")]
#[test_case(08, "VIII"; "eight")]
#[test_case(09, "IX"; "nine")]
#[test_case(10, "X"; "ten")]
fn test_to_string<T: num::PrimInt + num::Signed + ToString>(
  input: T,
  expected: &str,
) {
  assert_eq!(Roman::default().to_string(input).unwrap(), expected);
}

#[test_case("", 0; "empty")]
#[test_case("MMMDCCCLXXXVIII", 3888; "complicated")]
#[test_case("MMMCMXCIX", 3999; "maximum")]
#[test_case("I", 01; "one")]
#[test_case("II", 02; "two")]
#[test_case("III", 03; "three")]
#[test_case("IV", 04; "four")]
#[test_case("V", 05; "five")]
#[test_case("VI", 06; "six")]
#[test_case("VII", 07; "seven")]
#[test_case("VIII", 08; "eight")]
#[test_case("IX", 09; "nine")]
#[test_case("X", 10; "ten")]
fn test_from_str(input: &str, expected: i32) {
  assert_eq!(Roman::default().from_str::<i32>(input).unwrap(), expected);
}
