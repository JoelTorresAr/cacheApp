use crate::errors::{CacheErr, CacheResult};
use crate::CacheValue;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
#[derive(Debug)]
pub struct Cache {
    data: RwLock<HashMap<String, CacheValue>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn get<T>(&self, key: &str) -> CacheResult<Option<T>>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
    {
        let data = self.data.read();
        if let Some(cache) = data.get(key) {
            let result: T = serde_json::from_str(&cache.value)?;
            return Ok(Some(result));
        }
        Ok(None)
    }

    pub fn put<T>(&self, key: &str, value: T) -> CacheResult<()>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
    {
        let mut data = self.data.write();
        let serialize = serde_json::to_string(&value)?;
        data.insert(key.to_string(), CacheValue::new(serialize, None, None));
        Ok(())
    }

    pub fn set_group<T>(&self, key: &str, group: &str) -> CacheResult<()>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
    {
        let mut data = self.data.write();
        if let Some(cache) = data.get_mut(key) {
            cache.group = Some(group.to_string());
        }
        Ok(())
    }

    pub async fn remember<F, T, E>(&self, key: &str, hours: u64, func: F) -> CacheResult<T>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let data = self.data.read();
        if let Some(cache) = data.get(key) {
            if let Some(expires_at) = cache.expires_at {
                if SystemTime::now() <= expires_at {
                    let result: T = serde_json::from_str(&cache.value)?;
                    return Ok(result.clone());
                }
            } else {
                let result: T = serde_json::from_str(&cache.value)?;
                return Ok(result.clone());
            }
        }

        drop(data);

        let result = func
            .await
            .map_err(|e| CacheErr::ExternalError(e.to_string()))?;
        let mut data = self.data.write();
        let serialize = serde_json::to_string(&result)?;
        data.insert(
            key.to_string(),
            (CacheValue::new(serialize, Some(hours), None)),
        );

        Ok(result)
    }

    pub async fn remember_forever<F, T, E>(&self, key: &str, func: F) -> CacheResult<T>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
        F: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let data = self.data.read();
        if let Some(cache) = data.get(key) {
            let result: T = serde_json::from_str(&cache.value)?;
            return Ok(result.clone());
        }

        drop(data); // Liberar el Mutex antes de llamar a `func`

        let result = func
            .await
            .map_err(|e| CacheErr::ExternalError(e.to_string()))?;
        let mut data = self.data.write();
        let serialize = serde_json::to_string(&result)?;
        data.insert(key.to_string(), CacheValue::new(serialize, None, None));

        Ok(result)
    }

    pub fn forget(&self, key: &str) {
        let mut data = self.data.write();
        data.remove(key);
    }

    pub fn forget_group(&self, group: &str) {
        let mut data = self.data.write();
        let mut keys = Vec::new();
        for (key, cache) in data.iter() {
            if let Some(cache_group) = &cache.group {
                if cache_group == group {
                    keys.push(key.clone());
                }
            }
        }
        for key in keys {
            data.remove(&key);
        }
    }

    pub fn forget_all(&self) {
        let mut data = self.data.write();
        data.clear();
    }

    pub fn purge(&self) {
        let mut data = self.data.write();
        let mut keys = Vec::new();
        for (key, cache) in data.iter() {
            if let Some(expires_at) = cache.expires_at {
                if SystemTime::now() >= expires_at {
                    keys.push(key.clone());
                }
            }
        }
        for key in keys {
            data.remove(&key);
        }
    }
}
