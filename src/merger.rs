use ron::Value;

pub(crate) trait ConfigMerger {
    fn merge(left: Value, right: Value) -> Value;
}

pub(crate) struct DefaultConfigMerger;

impl ConfigMerger for DefaultConfigMerger {
    fn merge(left: Value, right: Value) -> Value {
        match (left, right) {
            (Value::Map(mut lm), Value::Map(rm)) => {
                for (rk, rv) in rm.iter() {
                    if let Some((lk, lv)) = lm.clone().iter().find(|(k, _)| {
                        **k == *rk
                    }) {
                        lm.insert(lk.clone(), DefaultConfigMerger::merge(lv.clone(), rv.clone()));
                    } else {
                        lm.insert(rk.clone(), rv.clone());
                    }
                }
                Value::Map(lm)
            }
            (_, r) => r
        }
    }
}
