use std::{collections::HashMap, str, sync::{Arc, Mutex, OnceLock}};


type GlobalMapType = HashMap<String, Arc<StorageCenter>>;
fn global_cnter_map() -> &'static Mutex<GlobalMapType> {
    static GLOBAL_LOCK: OnceLock<Mutex<GlobalMapType>> = OnceLock::new();
    GLOBAL_LOCK.get_or_init(|| {
        let map: GlobalMapType = HashMap::new();
        Mutex::new(map)
    })
}

enum StorageType {
    common,
    other,
}

impl StorageType {
    fn to_string(&self) -> String {
        match self {
            Self::common => "common".to_string(),
            Self::other => "other".to_string(),
            // _ => "unknown".to_string(),
        }
    }
}

struct StorageCenter {
    id: String,
    b_type: StorageType,
}

impl StorageCenter {
    fn center_with(id: String, b_type: StorageType) -> Arc<StorageCenter> {
        let mut map = global_cnter_map().lock().unwrap();
        let key = format!("{}_{}", id, b_type.to_string());
        let value = map.get(&key);
        match value {
            Some(center) => center.clone(),
            None => {
                let center = StorageCenter {
                    id: id,
                    b_type: b_type,
                };
                let ref_c = Arc::new(center);
                let result = ref_c.clone();
                map.insert(key, ref_c);
                result
            },
        }
    }
}