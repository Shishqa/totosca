## Release v0.1

Main goal: support TOSCA 1.3 parsing with error reporting
(like Puccini does).

### TODO

- [ ] [different TOSCA notations (short / long)](#different-notations)
- [ ] [structure versioning (for different TOSCA versions)](#versioning)
- [ ] [error reporting](#error-reporting)

### Done

## Similar projects

- https://github.com/emk/compose_yml
- https://github.com/faradayio/openapi-interfaces

## Notes

### Versioning

https://github.com/serde-rs/serde/issues/1470

- obake: Hard to use for big structures like TOSCA
  - lacks `inherit` for nested generic structures
  - Have to version all the TOSCA tree (even if only few structs differ)

- How to treat different versions? (always migrate?)

- Serde's `DeserializeSeed`?

### Different notations

- Notes on try-deserializing:
  https://github.com/serde-rs/serde/issues/464

- Impl of deserializing one of:
  https://github.com/faradayio/openapi-interfaces/commit/b1c9944a7ee277d42e8eb820d985feab30302ad3

- serde_with `PickFirst`

### Error reporting

- serde_yaml does not support location spans :(

- https://github.com/dtolnay/serde-yaml/pull/201

- https://www.reddit.com/r/learnrust/comments/10s0luu/reporting_multiple_validation_errors_when_reading/

- https://docs.rs/format_serde_error/latest/format_serde_error/

- Storing unknown fields:
    ```rust
    struct Test {
        a: String,
        b: String,
        #[serde(flatten)]
        unk: Map<String, serde_yaml::Value>,
    }
    ```

- `TryParse`
    ```rust
    #[derive(Debug)]
    pub enum TryParse<T> {
        Parsed(T),
        Unparsed(ParsingError),
        NotPresent
    }

    impl<'de, T: DeserializeOwned> Deserialize<'de> for TryParse<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            match Option::<serde_yaml::Value>::deserialize(deserializer)? {
                None => Ok(TryParse::NotPresent),
                Some(value) => match T::deserialize(&value) {
                    Ok(t) => Ok(TryParse::Parsed(t)),
                    Err(e) => Ok(TryParse::Unparsed(ParsingError {
                        value,
                        location: e.location(),
                        error: e.to_string(),
                    })),
                },
            }
        }
    }
    ```
