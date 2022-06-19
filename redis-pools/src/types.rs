use bb8_redis::redis::{
    from_redis_value, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AbParams {
    pub fill_a: f64,
    pub fill_b: f64,
    pub show_a: f64,
    pub show_b: f64,
    pub click_a: f64,
    pub click_b: f64,
}

impl AbParams {
    fn from(map: HashMap<String, f64>) -> Self {
        Self {
            fill_a: map.get("fill_a").unwrap_or(&0.0).clone(),
            fill_b: map.get("fill_b").unwrap_or(&0.0).clone(),
            show_a: map.get("show_a").unwrap_or(&0.0).clone(),
            show_b: map.get("show_b").unwrap_or(&0.0).clone(),
            click_a: map.get("click_a").unwrap_or(&0.0).clone(),
            click_b: map.get("click_b").unwrap_or(&0.0).clone(),
        }
    }
}

impl FromRedisValue for AbParams {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let map: HashMap<String, f64> = from_redis_value(v)?;
        Ok(AbParams::from(map))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdIdExpCfg {
    pub ad_id: i64,
    pub version: String,
    pub cg_user: String,
    pub eg_user: String,
    pub eg_action_id: String,
    pub main_action_id: String,
    pub exp_action_value: f64,
    pub main_action_value: f64,
}

impl FromRedisValue for AdIdExpCfg {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let v: Vec<u8> = from_redis_value(v)?;
        let data = serde_json::from_slice(v.as_slice()).unwrap_or(Default::default());
        Ok(data)
    }
}

impl ToRedisArgs for AdIdExpCfg {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes())
    }
}
