## Error reporting

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
