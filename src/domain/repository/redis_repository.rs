use std::error::Error;
use redis::{Commands, RedisError, Connection, FromRedisValue, ToRedisArgs};

pub struct RedisRepository {
    conn: Connection
}

impl RedisRepository {
    pub fn new() -> Result<Self, RedisError> {
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        Ok(RedisRepository {
            conn: client.get_connection()?
        })
    }

    pub fn get<'a, T>(&mut self, key: String) -> Result<T, Box<dyn Error>> where T: FromRedisValue {
        let value: T = self.conn.get(key)?;
        Ok(value)
    }

    pub fn set<'a, T>(&mut self, key: String, value: T) -> Result<String, Box<dyn Error>> where T: ToRedisArgs {
        let _: () = self.conn.set(key.clone(), value)?;
        Ok(key.clone())
    }

    pub fn del(&mut self, key: String) -> Result<(), Box<dyn Error>> {
        self.conn.del(key.clone())?;
        Ok(())
    }

    pub fn get_pending_images(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        match self.get::<String>("pending_images".to_string()).ok() {
            Some(val) => {
                Ok(serde_json::from_str(val.as_str())?)
            },
            None => Ok(vec![])
        }
    }

    pub fn set_pending_images(&mut self, pending_images: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.set("pending_images".to_string(), json!(pending_images).to_string())?;
        Ok(())
    }
}