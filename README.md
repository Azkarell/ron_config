# A simple tool to load configuration from file [![Rust](https://github.com/Azkarell/ron_config/actions/workflows/rust.yml/badge.svg)](https://github.com/Azkarell/ron_config/actions/workflows/rust.yml)


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
