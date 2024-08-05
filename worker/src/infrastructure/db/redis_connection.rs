use r2d2::Pool;
use redis::{Client, Connection};


pub fn establish_connection(connection: &str) -> Pool<Connection> {
    let client = Client::open(connection).unwrap();
    let manager = r2d2_redis::RedisConnectionManager::new(client);
    let pool = r2d2::Pool::builder().build(client).unwrap();
    pool
}
