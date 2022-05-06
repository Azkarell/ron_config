use ron::Value;

pub(crate) trait PathResolver {
    fn find(config: &Value, path: ConfigPath) -> Option<&Value>;
}

pub(crate) struct DefaultPathResolver;

impl DefaultPathResolver {
    fn find_internal(cur: &Value, left_segments: Vec<String>) -> Option<&Value> {

        if let Some(cur_seg) = left_segments.first() {
            let map = match cur {
                Value::Map(m) => m,
                _ => return None
            };
            if let Some((_, v)) = map.iter().find(|(k, _)| {
                match k {
                    Value::String(str) => {
                        *str == *cur_seg
                    }
                    _ => false
                }
            }) {
                return DefaultPathResolver::find_internal(v, left_segments[1..].to_vec());
            }
        }
        Some(cur)
    }
}

impl PathResolver for DefaultPathResolver {
    fn find(value: &Value, path: ConfigPath) -> Option<&Value> {
        match value {
            Value::Map(_) => {
                DefaultPathResolver::find_internal(value, path.segments)
            }
            _ => None
        }
    }
}



#[derive(Debug, Clone)]
pub struct ConfigPath {
    pub(crate) segments: Vec<String>,
}

impl ToString for ConfigPath {
    fn to_string(&self) -> String {
        self.segments.join(".")
    }
}

impl From<String> for ConfigPath {
    fn from(s: String) -> ConfigPath {
        ConfigPath {
            segments: s.split('.').map(|s| s.to_string()).collect()
        }
    }
}

impl<'a> From<&'a str> for ConfigPath {
    fn from(s: &'a str) -> ConfigPath {
        ConfigPath {
            segments: s.split('.').map(|s| s.to_string()).collect()
        }
    }
}

impl From<Vec<String>> for ConfigPath {
    fn from(v: Vec<String>) -> ConfigPath {
        ConfigPath {
            segments: v
        }
    }
}

impl From<Vec<&str>> for ConfigPath {
    fn from(v: Vec<&str>) -> ConfigPath {
        ConfigPath {
            segments: v.iter().map(|s| s.to_string()).collect()
        }
    }
}


pub trait SegmentedPath {
    fn get_segments(&self) -> Vec<String>;
}

impl<T: ToString> SegmentedPath for T {
    fn get_segments(&self) -> Vec<String> {
        self.to_string().split('.').map(|p| p.to_owned()).collect()
    }
}

impl From<&dyn SegmentedPath> for ConfigPath {
    fn from(p: &dyn SegmentedPath) -> Self {
        ConfigPath {
            segments: p.get_segments()
        }
    }
}