use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;

use crate::{bidder::Bidder, types::*};
use crate::bidder::MyBidder;

pub const DEFAULT_BIDDER_ID: u64 = 1;

#[async_trait]
trait AdaptedBidder {
    async fn request_bid(&self, http_client: &Client, ctx: &AdxContext<'_>, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<BidderResponse>;
}


pub fn bidder_mapping() -> HashMap<u64, BidderAdapter> {
    let mut bidders = HashMap::new();
    bidders.insert(1, BidderAdapter::new(Box::new(MyBidder::new())));
    bidders.insert(2, BidderAdapter::new(Box::new(MyBidder::new())));
    bidders
}

pub struct BidderAdapter {
    pub bidder: Box<dyn Bidder>,
}

impl BidderAdapter {
    pub fn new(bidder: Box<dyn Bidder>) -> BidderAdapter {
        Self { bidder }
    }
}
//
// #[async_trait]
// impl AdaptedBidder for BidderAdapter {
//     async fn request_bid(&self, http_client: &Client, ctx: & AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<BidderResponse> {
//         let requestData = self.bidder.make_requests(ctx, ad_campaign, ad_source).await?;
//         let call_info = self.do_request(http_client, ctx, ad_campaign, ad_source).await?;
//         let bid = self.bidder.make_bids(ctx, ad_campaign, &call_info.response).await?;
//         // todo!()
//         Ok(BidderResponse {})
//     }
// }

impl BidderAdapter {
    pub async fn request_bid(&self, http_client: &Client, ctx: &AdxContext<'_>, ad_campaign: &AdCampaign, ad_source: AdSource) -> Result<BidderResponse> {
        let request_data = self.bidder.make_requests(ctx, ad_campaign, &ad_source).await?;
        let call_info = self.do_request(http_client, ctx, ad_campaign, &ad_source).await?;
        let bid = self.bidder.make_bids(ctx, ad_campaign, &call_info.response_data).await?;
        // todo!()
        Ok(BidderResponse { ad_campaign: todo!(), ad_source, bid_response: todo!() })
    }

    async fn do_request(&self, http_client: &Client, ctx: &AdxContext<'_>, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpCallInfo> {
        Ok(HttpCallInfo {
            // request: HttpRequestData {
            //      method: "".to_string(),
            //     uri: "".to_string(),
            //     body: None,
            //  },
            response_data: HttpResponseData {
                status_code: 200,
                body: vec![],
            },
            // process_time: 1,
        })
    }
}