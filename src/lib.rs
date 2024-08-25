pub mod app;
pub use app::*;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::thread;
    use std::time::Duration;
    use tokio;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct User {
        pub name: String,
        pub email: String,
    }

    pub async fn get_user() -> Result<User, Box<dyn std::error::Error>> {
        thread::sleep(Duration::from_secs(1));
        Ok(User {
            name: "Joel Torres".to_string(),
            email: "djoel_torres@hotmail.com".to_string(),
        })
    }

    #[tokio::test]
    async fn test_remember() {
        let cache = Cache::new();
        let fun = get_user();
        let result = cache.remember("test_remember", 1, fun).await.unwrap();
        assert_eq!(result.name, "Joel Torres");
    }

    #[tokio::test]
    async fn test_remember_forever() {
        let cache = Cache::new();
        let fun = get_user();
        let result = cache
            .remember_forever("test_remember_forever", fun)
            .await
            .unwrap();
        assert_eq!(result.name, "Joel Torres");
    }

    #[tokio::test]
    async fn test_get() {
        let cache = Cache::new();
        let result = cache.get::<String>("test_get").unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_put() {
        let cache = Cache::new();
        cache.put("test_put", "Hello World".to_string()).unwrap();
        let result = cache.get::<String>("test_put").unwrap();
        assert_eq!(result.unwrap(), "Hello World");
    }

    #[tokio::test]
    async fn test_forget() {
        let cache = Cache::new();
        cache.put("test_forget", "Hello World".to_string()).unwrap();
        let result = cache.get::<String>("test_forget").unwrap();
        assert_eq!(result.unwrap(), "Hello World");

        cache.forget("test_forget");
        let result = cache.get::<String>("test_forget").unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_forget_all() {
        let cache = Cache::new();
        cache
            .put("test_forget_all", "Hello World".to_string())
            .unwrap();
        let result = cache.get::<String>("test_forget_all").unwrap();
        assert_eq!(result.unwrap(), "Hello World");

        cache.forget_all();
        let result = cache.get::<String>("test_forget_all").unwrap();
        assert_eq!(result, None);
    }
}
