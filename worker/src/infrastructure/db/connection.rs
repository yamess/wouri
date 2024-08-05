use deadpool_redis::{Config, Pool, Runtime};

pub fn establish_connection(connection_str: &str, pool_size: usize) -> Pool {
    let cfg = Config::from_url(connection_str);
    let pool = cfg
        .builder()
        .unwrap()
        .max_size(pool_size)
        .wait_timeout(Some(std::time::Duration::from_secs(30)))
        .runtime(Runtime::Tokio1)
        .build()
        .expect("Failed to create pool");
    pool
}
