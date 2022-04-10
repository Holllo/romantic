# Romantic

> Roman numeral toolkit

## API

For full documentation see [docs.rs].

[docs.rs]: https://docs.rs/romantic

## Examples

Using the default Roman numeral system.

```rust
use romantic::Roman;

let roman = Roman::default();

assert_eq!(roman.to_string(2022).unwrap(), "MMXXII");
assert_eq!(roman.from_str::<i32>("MMXXII").unwrap(), 2022);

// The default Roman numeral has a maximum of 3999.
assert!(roman.to_string(4000).is_err());
```

Using your own custom character set.

```rust
use romantic::Roman;

// The order of characters in the array determines their value.
// Here, A equals 1 and B equals 5.
let custom = Roman::new(&['A', 'B']);

assert_eq!(custom.to_string(6).unwrap(), "BA");
assert_eq!(custom.from_str::<i32>("BA").unwrap(), 6);

// With only 2 characters, the maximum value you can get is 8
// (the equivalent of VIII). To increase the maximum range, use
// more characters.
assert!(custom.to_string(9).is_err());
```

## License

This project is licensed under either of [Apache License, Version 2.0](https://github.com/Holllo/romantic/blob/main/LICENSE-Apache) or [MIT license](https://github.com/Holllo/romantic/blob/main/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
