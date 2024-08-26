# CACHE APP: README.md
# CACHE APP IS A STABLE CACHE REMEMBER CRATE MIGRATION

| Resource          | Link                                                                                                                              |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------|
| Crate version     | [![Crates.io](https://img.shields.io/crates/v/wkhtmlapp?color=warning&style=plastic)](https://crates.io/crates/cacheapp)          |
| Documentation     | [![Documentation](https://docs.rs/cache_remember/badge.svg)](https://github.com/JoelTorresAr/cacheapp.git)                        |
| LICENSE           | [![LICENSE](https://img.shields.io/crates/l/cacheapp?style=plastic)](./LICENSE)


Cache App is a simple caching library for rust that allows you to cache the result of a function call for a given amount of time.
Inspired in laravel's cache remember.

The remember function uses an async function as one of its parameters, which function must return a value that has Deserialize implemented, 
Serialize for serde. If it has a cached value, it returns the value without executing the function, otherwise it will execute the function 
and store the result in cache for future queries.

## [0.1.7] - 2024-08-25
Add forget_filter and forget_group_filter, which allow you to delete items using a function such as a filter

## [0.1.6] - 2024-08-25
Add set_group and forget_group.
For add items to group and delete all items into group

## [0.1.3] - 2023-06-11
Changed Box dyn std::error::Error so that it now accepts functions that return any Error as long as it has the Display trait implemented.
Change Mutex to RwLock to allow multiple reads at the same time.

## [0.1.2] - 2023-06-11
REMOVE INNECESARY ASYNC AND RETURN RESULT IN forget(), forget_all() and purge() functions.
## EXAMPLE

```rust
    use cacheapp::Cache;
    use std::thread;
    use std::time::Duration;
    use serde::{Deserialize, Serialize};
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

    fn main() {
            // Create a new cache instance
            let cache = Cache::new();
            // Get the result of the function call
            let fun = get_user();
            let hours : u64 = 1;
            let result = cache.remember("test_remember", hours, fun).await.unwrap();
            println!("{:?}", result);

            //forget the cache
            cache.forget("test_remember");
            cache.forget_filter(|x| x.contains("test"));

            // remember forever
            let fun = get_user();
            let result = cache.remember_forever("test_remember", fun).await.unwrap();
            println!("{:?}", result);

            // set group
            cache.set_group("test_remember", "new_group");

            //forget group
            cache.forget_group("new_group");
            cache.forget_group_filter(|x| x.contains("new"));

            //forget all cache
            cache.forget_all();
            println!("{:?}", cache);

            //purge expired records in cache
            cache.purge();
    }
```