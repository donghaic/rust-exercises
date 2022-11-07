use anyhow::Result;
use async_trait::async_trait;
use reqwest::Method;

use crate::types::*;

#[async_trait]
pub trait Bidder {
    async fn make_requests(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpRequestData>;

    async fn make_bids(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, http_data: &HttpResponseData) -> Result<BidResponse>;
}


pub struct MyBidder {}

impl MyBidder {
    pub fn new() -> MyBidder {
        MyBidder {}
    }
}

#[async_trait]
impl Bidder for MyBidder {
    async fn make_requests(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, ad_source: &AdSource) -> Result<HttpRequestData> {
        Ok(HttpRequestData {
            method: Method::GET,
            uri: "".to_string(),
            body: vec![],
        })
    }

    async fn make_bids(&self, ctx: &AdxContext, ad_campaign: &AdCampaign, http_data: &HttpResponseData) -> Result<BidResponse> {
        Ok(BidResponse {
            id: "".to_string(),
            bidfloor: 0,
        })
    }
}