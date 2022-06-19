mod types;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::error::Error;

    use bb8_redis::redis::RedisResult;
    use bb8_redis::{bb8, redis::AsyncCommands, RedisConnectionManager};
    use crate::types::{AbParams, AdIdExpCfg};

    use super::*;

    #[tokio::test]
    async fn bb8_it_works() -> Result<(), Box<dyn Error>> {
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = bb8::Pool::builder().build(manager).await.unwrap();
        println!(" test pool");
        let mut conn = pool.get().await?;

        let res: RedisResult<()> = conn.set("key", "value").await;
        println!("set res = {:?}", res);

        let val1: String = conn.get("key").await?;
        println!("key = {:?}", val1);

        let missing_val: RedisResult<String> = conn.get("missing_key").await;
        println!("missing_key = {:?}", missing_val);

        let map: HashMap<String, i32> = conn.hgetall("my_hash").await?;
        println!("my_hash = {:?}", map);

        let map: RedisResult<HashMap<String, i32>> = conn.hgetall("my_hash").await;
        println!("my_hash2 = {:?}", map);
        Ok(())
    }

    #[tokio::test]
    async fn from_redis_value_hash() -> Result<(), Box<dyn Error>> {
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = bb8::Pool::builder().build(manager).await.unwrap();
        let mut conn = pool.get().await?;

        let ab_params: AbParams = conn.hgetall("miss_hash").await?;
        println!("miss_hash = {:?}", ab_params);

        let ab_params: AbParams = conn.hgetall("ab_params").await?;
        println!("ab_params = {:?}", ab_params);
        Ok(())
    }


    #[tokio::test]
    async fn from_redis_value_json() -> Result<(), Box<dyn Error>> {
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = bb8::Pool::builder().build(manager).await.unwrap();
        let mut conn = pool.get().await?;

        let mut ad_cfg = AdIdExpCfg::default();
        ad_cfg.ad_id = 1;
        ad_cfg.main_action_id = "1".into();
        ad_cfg.cg_user = "a".into();

        conn.set("adid:cfg", ad_cfg).await?;


        let ad_cfg2: AdIdExpCfg = conn.get("adid:cfg").await?;
        println!("ad_cfg2 = {:?}", ad_cfg2);

        Ok(())
    }
}
