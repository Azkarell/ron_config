# A simple tool to load configuration from file [![Rust](https://github.com/Azkarell/ron_config/actions/workflows/rust.yml/badge.svg)](https://github.com/Azkarell/ron_config/actions/workflows/rust.yml)

## Issues
It is currently not possible to use enums. This is caused by how ron handles enums in its Value struct. Either i write something on my own (which is currently not planned because workaround with strings is possible but cumbersome) or I hope ron improves enum handling intern. As far as i can tell they are working on an improved version. i will wait and see if this is usable.

## Supported file formats

Currently only ron is supported (as the name suggests :))


## Simple usage

```rust
    use ron_config::ConfigBuilder;
    let config = ConfigBuilder::new()
      .str("(foo: \"bar\")")
      .build();
    assert_eq!(config.try_get::<String>("foo".into()).unwrap(), "bar");
```
