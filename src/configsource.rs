use std::fs;
use ron::Value;

pub trait ConfigSource {
    fn get_value(&self) -> Value;
}

pub(crate) struct FileConfigSource {
    path: String,
}

impl FileConfigSource {
    pub fn new(path: &str) -> FileConfigSource {
        FileConfigSource {
            path: path.to_owned()
        }
    }
}

impl ConfigSource for FileConfigSource {
    fn get_value(&self) -> Value {
        let file = fs::read_to_string(&self.path).expect("Failed to read file");
        ron::from_str(&file).expect("Failed to parse config file")
    }
}

pub(crate) struct StringConfigSource {
    string: String,
}

impl StringConfigSource {
    pub fn new(string: &str) -> StringConfigSource {
        StringConfigSource {
            string: string.to_owned()
        }
    }
}

impl ConfigSource for StringConfigSource {
    fn get_value(&self) -> Value {
        ron::from_str(&self.string).expect("Failed to parse config string")
    }
}