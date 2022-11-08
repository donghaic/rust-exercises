#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use adx_base::{AdxDB, Exchange, indexer::Indexer, types::{AdxContext, BidRequest}};
use adx_base::errors::*;
use adx_base::types::Media;

#[tokio::main]
async fn main() -> Result<()> {
    let adx_db = AdxDB::new(Indexer::new());
    let exchange = Exchange::new(&adx_db);

    let cache = Cache::builder()
        // Time to live (TTL): 30 minutes
        .time_to_live(Duration::from_secs(30 * 60))
        // Create the cache.
        .build();

    let a = cache.insert(1, Arc::new(Media { id: 0, name: "".to_string() })).await;

    let media = cache.get(&1).unwrap();
    let b = media;
    println!("{:?}", a);


    let adslot = adx_db.get_adslot_by_id(1).map_err(|_| BizError::NotFound)?;
    let media = adx_db.get_media_by_id(2).map_err(|_| BizError::NotFound)?;

    let mut ctx = AdxContext {
        req_id: "".to_string(),
        adslot: adslot,
        media: media,
        bid_request: BidRequest { id: "".to_string(), device_id: "".to_string() },
    };

    let _result = exchange.auction(&mut ctx).await;
    println!("Hello, world!");

    Ok(())
}
