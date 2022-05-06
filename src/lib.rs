use std::fmt::Debug;
use std::fs;
use std::path::Path;
use ron::{Value};
use serde::de::DeserializeOwned;
use crate::configpath::{ConfigPath, DefaultPathResolver, PathResolver};
use crate::configsource::{ConfigSource, FileConfigSource, StringConfigSource};
use crate::merger::{ConfigMerger, DefaultConfigMerger};

mod merger;
mod configpath;
mod configsource;

#[derive(Debug)]
pub struct Config {
    config: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ConfigValue {
    serialized: String,
}


impl Config {
    pub fn from_value(value: Value) -> Self {
        Self {
            config: value,
        }
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let file = std::fs::read_to_string(path).expect("Could not read config file");
        let config = ron::from_str(&file).expect("Could not parse config file");
        Config {
            config,

        }
    }

    pub fn from_string(val: &str) -> Self {
        let config = ron::from_str(val).expect("Could not parse config file");
        Config {
            config,

        }
    }

    /// # Example
    /// ```
    /// use ron_config::Config;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, Debug)]
    /// pub struct MyConfig {
    ///     pub number: f32,
    ///    pub name: String,
    /// }
    /// let mut config = Config::from_string("MyConfig(number: 1.0, name: \"test\")");
    ///
    /// let my_config = config.try_get::<MyConfig>(".".into()).unwrap();
    ///
    /// assert_eq!(my_config.number, 1.0);
    /// assert_eq!(my_config.name, "test");
    /// ```
    pub fn try_get<T: DeserializeOwned>(&self, path: ConfigPath) -> Option<T> {
        let value = DefaultPathResolver::find(&self.config, path)?;

        value.clone().into_rust::<T>().ok()
    }
    /// # Example
    /// ```
    /// use ron_config::Config;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, Debug)]
    /// pub struct MyConfig {
    ///     pub number: f32,
    ///    pub name: String,
    /// }
    /// #[derive(Deserialize, Debug)]
    /// pub struct MyConfig2 {
    ///     pub number2: f32,
    ///    pub name1: String,
    /// }
    /// let mut config1 = Config::from_string("(config1: MyConfig(number: 1.0, name: \"test\"))");
    /// let config2 = Config::from_string("(config2: MyConfig2(number2: 2.0, name1: \"test2\"))");
    /// config1.merge(config2);
    ///
    /// let my_config1 = config1.try_get::<MyConfig>("config1".into()).unwrap();
    /// let my_config2 = config1.try_get::<MyConfig2>("config2".into()).unwrap();
    ///
    /// assert_eq!(my_config2.number2, 2.0);
    /// assert_eq!(my_config2.name1, "test2");
    ///
    /// assert_eq!(my_config1.number, 1.0);
    /// assert_eq!(my_config1.name, "test");
    /// ```
    pub fn merge(&mut self, other: Config) {
        self.config = DefaultConfigMerger::merge(self.config.clone(), other.config);
    }
}


pub struct ConfigBuilder {
    sources: Vec<Box<dyn ConfigSource>>,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            sources: Vec::new()
        }
    }

    pub fn file(&mut self, path: &str) -> &mut ConfigBuilder {
        self.add_source(Box::new(FileConfigSource::new(path)))
    }

    pub fn str(&mut self, s: &str) -> &mut ConfigBuilder {
        self.add_source(Box::new(StringConfigSource::new(s)))
    }

    pub fn add_source(&mut self, source: Box<dyn ConfigSource>) -> &mut ConfigBuilder {
        self.sources.push(source);
        self
    }


    /// # Example
    /// ```
    /// use ron_config::ConfigBuilder;
    /// let config = ConfigBuilder::new()
    ///   .str("(foo: \"bar\")")
    ///  .build();
    /// assert_eq!(config.try_get::<String>("foo".into()).unwrap(), "bar");
    /// ```
    pub fn build(&self) -> Config {
        let mut config = Config::from_value(Value::Unit);
        for source in &self.sources {
            config.merge(Config::from_value(source.get_value()));
        }
        config
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_config_path() {
        let p = ConfigPath::from("a.b.c");
        assert_eq!(p.segments, vec!["a", "b", "c"]);
        assert_eq!(p.to_string(), "a.b.c");
    }

    #[test]
    fn test_config_path_from_string() {
        let p = ConfigPath::from("a.b.c".to_string());
        assert_eq!(p.segments, vec!["a", "b", "c"]);
        assert_eq!(p.to_string(), "a.b.c");
    }

    #[test]
    fn test_config_path_from_str() {
        let p = ConfigPath::from("a.b.c");
        assert_eq!(p.segments, vec!["a", "b", "c"]);
        assert_eq!(p.to_string(), "a.b.c");
    }

    #[test]
    fn test_config_path_from_vec() {
        let p = ConfigPath::from(vec!["a", "b", "c"]);
        assert_eq!(p.segments, vec!["a", "b", "c"]);
        assert_eq!(p.to_string(), "a.b.c");
    }


    #[test]
    fn test_config_path_from_vec_of_str() {
        let p = ConfigPath::from(vec!["a", "b", "c"]);
        assert_eq!(p.segments, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_config_string() {
        let config = Config::from_value(ron::from_str("(foo: \"bar\")").expect("value"));
        let t: String = config.try_get("foo".into()).unwrap();
        assert_eq!(t, "bar");
    }

    #[test]
    fn test_config_number() {
        let config = Config::from_value(ron::from_str("(foo: 1)").expect("value"));
        let t: i32 = config.try_get("foo".into()).unwrap();
        assert_eq!(t, 1);
    }

    #[test]
    fn test_config_bool() {
        let config = Config::from_value(ron::from_str("(foo: true)").expect("value"));
        let t: bool = config.try_get("foo".into()).unwrap();
        assert_eq!(t, true);
    }

    #[test]
    fn test_config_option_some() {
        let config = Config::from_value(ron::from_str("(foo: Some(1))").expect("value"));
        let t: Option<i32> = config.try_get("foo".into()).unwrap();
        assert_eq!(t, Some(1));
    }

    #[test]
    fn test_config_option_none() {
        let config = Config::from_value(ron::from_str("(foo: None)").expect("value"));
        let t: Option<i32> = config.try_get("foo".into()).unwrap();
        assert_eq!(t, None);
    }

    #[test]
    fn test_config_vec() {
        let config = Config::from_string("(foo: [1, 2, 3])");
        let t: Vec<i32> = config.try_get("foo".into()).unwrap();
        assert_eq!(t, vec![1, 2, 3]);
    }

    #[test]
    fn test_config_merge() {
        let mut config = Config::from_string("(foo: 1)");
        let config2 = Config::from_string("(bar: 2)");
        config.merge(config2);
        let t: i32 = config.try_get("foo".into()).unwrap();
        assert_eq!(t, 1);
        let t: i32 = config.try_get("bar".into()).unwrap();
        assert_eq!(t, 2);
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Bar {
        t: i32,
    }

    #[test]
    fn test_config_merge_complex() {
        let mut config = Config::from_string("(foo: bar(t: 1))");
        let config2 = Config::from_string("(bar: 2)");
        let config3 = Config::from_string("(baz: 3)");
        config.merge(config2);
        config.merge(config3);
        let t: Bar = config.try_get("foo".into()).unwrap();
        assert_eq!(t, Bar {
            t: 1
        });
        let t: i32 = config.try_get("foo.t".into()).unwrap();
        assert_eq!(t, 1);
        let t: i32 = config.try_get("bar".into()).unwrap();
        assert_eq!(t, 2);
        let t: i32 = config.try_get("baz".into()).unwrap();
        assert_eq!(t, 3);
    }
}



