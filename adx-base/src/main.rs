use adx_base::{AdxDB,Exchange, types::{AdxContext, Adslot, BidRequest, Media}, indexer::Indexer};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let adx_db = AdxDB::new(Indexer::new());
    let exchange = Exchange::new(adx_db);

    let mut ctx = AdxContext {
        req_id: "".to_string(),
        ad_slot: &Adslot { id: 0 },
        media: &Media { id: 0 },
        bid_request: BidRequest { id: "".to_string(), device_id: "".to_string() },
        // err_code: RefCell::new(RwLock::new(Default::default())),
    };

    let _result = exchange.auction(&mut ctx).await;
    println!("Hello, world!");

    Ok(())
}
