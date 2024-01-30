## Grammar versioning

### Serde?

https://github.com/serde-rs/serde/issues/1470

- obake: Hard to use for big structures like TOSCA
  - lacks `inherit` for nested generic structures
  - Have to version all the TOSCA tree (even if only few structs differ)

- How to treat different versions? (always migrate?)

- Serde's `DeserializeSeed`?
