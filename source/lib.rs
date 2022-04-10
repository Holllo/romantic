#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

//! # Romantic
//!
//! Using the default Roman numeral system.
//!
//! ```rust
//! use romantic::Roman;
//!
//! let roman = Roman::default();
//!
//! assert_eq!(roman.to_string(2022).unwrap(), "MMXXII");
//! assert_eq!(roman.from_str::<i32>("MMXXII").unwrap(), 2022);
//!
//! // The default Roman numeral system has a maximum of 3999.
//! assert!(roman.to_string(4000).is_err());
//! ```
//!
//! Using your own custom character set.
//!
//! ```rust
//! use romantic::Roman;
//!
//! // The order of characters in the array determines their value.
//! // Here, A equals 1 and B equals 5.
//! let custom = Roman::new(&['A', 'B']);
//!
//! assert_eq!(custom.to_string(6).unwrap(), "BA");
//! assert_eq!(custom.from_str::<i32>("BA").unwrap(), 6);
//!
//! // With only 2 characters, the maximum value you can get is 8
//! // (the equivalent of VIII). To increase the maximum range, use
//! // more characters.
//! assert!(custom.to_string(9).is_err());
//! ```

use std::collections::HashMap;

/// All possible errors that can occur during conversion.
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
  /// The error when converting from a [`usize`] to [`num::PrimInt`] fails.
  #[error("Conversion error with generic integer")]
  GenericConversion,

  /// The error when an input character does not have an associated value in the
  /// [`Roman`] set.
  #[error("Invalid character \"{0}\" encountered")]
  InvalidCharacter(char),

  /// The error when an input magnitude does not have an associated character in
  /// the [`Roman`] set.
  #[error("Missing magnitude \"{0}\" for input number")]
  MissingMagnitude(usize),

  /// The error when an input number is negative.
  #[error("Input number cannot be negative")]
  NegativeNumber,

  /// The error when calculating an integer would cause an overflow.
  #[error("Operation would cause overflow")]
  Overflow,
}

/// The main struct for [`romantic`][crate].
#[derive(Debug)]
pub struct Roman {
  /// The mapping of a character to its corresponding magnitude (ie. 1 = 'I').
  character_magnitude_map: HashMap<char, usize>,

  /// The mapping of a magnitude to its corresponding character (ie. 'I' = 1).
  magnitude_character_map: HashMap<usize, char>,
}

impl Default for Roman {
  fn default() -> Self {
    Self::new(&['I', 'V', 'X', 'L', 'C', 'D', 'M'])
  }
}

impl Roman {
  /// Creates a new [`Roman`] using the characters in `character_set`.
  ///
  /// The order of the `character_set` determines their magnitude, for example
  /// using the default numeral system:
  ///
  /// | Index | Magnitude | Character |
  /// |-------|-----------|-----------|
  /// | 0     | 1         | 'I'       |
  /// | 1     | 5         | 'V'       |
  /// | 2     | 10        | 'X'       |
  /// | 3     | 50        | 'L'       |
  /// | 4     | 100       | 'C'       |
  /// | 5     | 500       | 'D'       |
  /// | 6     | 1000      | 'M'       |
  /// | ...   | ...       | ...       |
  ///
  /// ## Example
  ///
  /// ```rust
  /// use romantic::Roman;
  ///
  /// let roman = Roman::default();
  /// assert_eq!(roman.to_string(9).unwrap(), "IX");
  /// assert_eq!(roman.from_str::<i32>("IX").unwrap(), 9);
  ///
  /// let custom = Roman::new(&['A', 'B', 'C']);
  /// assert_eq!(custom.to_string(9).unwrap(), "AC");
  /// assert_eq!(custom.from_str::<i32>("AC").unwrap(), 9);
  /// ```
  pub fn new(character_set: &[char]) -> Self {
    let mut character_magnitude_map = HashMap::new();
    let mut magnitude_character_map = HashMap::new();

    let values = [1, 5];
    let modulo = values.len();

    let mut magnitude = 1;

    for (index, &character) in character_set.iter().enumerate() {
      if index > 0 && index % modulo == 0 {
        magnitude *= 10;
      }

      let value = magnitude * values[index % modulo];
      character_magnitude_map.insert(character, value);
      magnitude_character_map.insert(value, character);
    }

    Self {
      character_magnitude_map,
      magnitude_character_map,
    }
  }

  /// Converts a [`str`] to a generic integer [`num::PrimInt`].
  ///
  /// ## Example
  ///
  /// ```rust
  /// use romantic::Roman;
  ///
  /// let roman = Roman::default();
  /// assert_eq!(roman.from_str::<i32>("IX").unwrap(), 9);
  ///
  /// let custom = Roman::new(&['A', 'B', 'C']);
  /// assert_eq!(custom.from_str::<i32>("AC").unwrap(), 9);
  /// ```
  pub fn from_str<T: num::PrimInt>(
    &self,
    input: &str,
  ) -> Result<T, ConversionError> {
    let mut characters = input.chars().peekable();
    let mut result = T::zero();

    while let Some(character) = characters.next() {
      let value = self
        .character_magnitude_map
        .get(&character)
        .ok_or(ConversionError::InvalidCharacter(character))?;

      let generic_value =
        T::from(*value).ok_or(ConversionError::GenericConversion)?;

      if let Some(next) = characters.peek() {
        let next = self.character_magnitude_map.get(next);

        let subtract = match next {
          Some(&next_value) => {
            (value * 5 == next_value) || (value * 10 == next_value)
          }
          None => false,
        };

        if subtract {
          result = result
            .checked_sub(&generic_value)
            .ok_or(ConversionError::Overflow)?;
          continue;
        }
      }

      result = result
        .checked_add(&generic_value)
        .ok_or(ConversionError::Overflow)?;
    }

    Ok(result)
  }

  /// Converts a generic integer [`num::PrimInt`] to a [`String`].
  ///
  /// ## Example
  ///
  /// ```rust
  /// use romantic::Roman;
  ///
  /// let roman = Roman::default();
  /// assert_eq!(roman.to_string(9).unwrap(), "IX");
  ///
  /// let custom = Roman::new(&['A', 'B', 'C']);
  /// assert_eq!(custom.to_string(9).unwrap(), "AC");
  /// ```
  pub fn to_string<T: num::PrimInt + ToString>(
    &self,
    number: T,
  ) -> Result<String, ConversionError> {
    if number < T::zero() {
      return Err(ConversionError::NegativeNumber);
    }

    let mut result = String::new();

    for (index, digit) in number.to_string().chars().rev().enumerate() {
      // Skip any zeroes in the number since we don't have to do anything for it.
      if digit == '0' {
        continue;
      }

      // Safe to unwrap since this can't be anything other than a 1..=9 digit.
      let digit = digit.to_digit(10).unwrap() as usize;
      let magnitude = num::pow::pow(10, index);

      // Get all the units for this magnitude and intentionally leave them as
      // `Result`s here. Since the default Roman numeral set only goes up to
      // 4000, we can't require unit 5 and 10 for magnitude 1000 (5000, 10000).
      // So once we go to add them to the result string, only then use `Result?`
      // to get their characters.
      let value_of_character = |m: usize| -> Result<String, ConversionError> {
        self
          .magnitude_character_map
          .get(&m)
          .map(ToString::to_string)
          .ok_or(ConversionError::MissingMagnitude(m))
      };

      let unit_1 = value_of_character(magnitude);
      let unit_5 = value_of_character(magnitude * 5);
      let unit_10 = value_of_character(magnitude * 10);

      // Map the digit to its character, using magnitude 1 as examples.
      result += &match digit {
        // 1 through 3 equals I, II, III.
        1..=3 => unit_1?.repeat(digit),

        // 4 equals IV (note the reversed formatting).
        4 => format!("{}{}", unit_5?, unit_1?),

        // 5 equals V.
        5 => unit_5?,

        // 6 through 8 equals VI, VII, VIII (also reversed).
        6..=8 => format!("{}{}", unit_1?.repeat(digit - 5), unit_5?),

        // 9 equals IX (also reversed).
        9 => format!("{}{}", unit_10?, unit_1?),

        _ => unreachable!(),
      };
    }

    Ok(result.chars().rev().collect())
  }
}
