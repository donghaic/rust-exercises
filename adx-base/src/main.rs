#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Result;

use adx_base::{AdxDB, Exchange, indexer::Indexer, types::{AdxContext, BidRequest}};
use adx_base::errors::*;

#[tokio::main]
async fn main() -> Result<()> {
    let adx_db = AdxDB::new(Indexer::new());
    let exchange = Exchange::new(&adx_db);


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
