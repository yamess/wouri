use deadpool_redis::{Config, Pool, Runtime};

pub fn establish_connection(connection_str: &str) -> Pool {
    let cfg = Config::from_url(connection_str);
    let pool = cfg
        .builder()
        .unwrap()
        .max_size(15)
        .wait_timeout(Some(std::time::Duration::from_secs(30)))
        .runtime(Runtime::Tokio1)
        .build()
        .expect("Failed to create pool");
    pool
}
