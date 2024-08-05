use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub redis_connection_string: String,
}
impl RedisConfig {
    pub fn new() -> Self {
        envy::from_env::<Self>().unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RMQConfig {
    pub rmq_connection_string: String,
}
impl RMQConfig {
    pub fn new() -> Self {
        envy::from_env::<Self>().unwrap()
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub redis: RedisConfig,
    pub rmq: RMQConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            redis: RedisConfig::new(),
            rmq: RMQConfig::new(),
        }
    }
}