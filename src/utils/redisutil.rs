use r2d2_redis::r2d2::Pool;
use r2d2_redis::{r2d2, RedisConnectionManager};

pub fn get_redis_pool()->Pool<RedisConnectionManager>{
    let maneger = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool =  r2d2::Pool::builder().build(maneger).unwrap();
    pool
}