use std::collections::HashMap;
use std::time::Duration;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::{Client, Method, RequestBuilder};

use crate::{bidder::Bidder, types::*};
use crate::bidder::MyBidder;

pub const DEFAULT_BIDDER_ID: u64 = 1;
const REQUEST_TIMEOUT: Duration = Duration::from_millis(200);


pub fn bidder_mapping() -> HashMap<u64, BidderDispatcher> {
    let mut bidders = HashMap::new();
    bidders.insert(1, BidderDispatcher::new(Box::new(MyBidder::new())));
    bidders.insert(2, BidderDispatcher::new(Box::new(MyBidder::new())));
    bidders
}

pub struct BidderDispatcher {
    pub bidder: Box<dyn Bidder>,
}

impl BidderDispatcher {
    pub fn new(bidder: Box<dyn Bidder>) -> BidderDispatcher {
        Self { bidder }
    }
}


impl BidderDispatcher {
    pub async fn request_bid(&self, http_client: &Client, ctx: &AdxContext<'_>, ad_campaign: &AdCampaign, ad_source: AdSource) -> Result<BidderResponse> {
        let request_data = self.bidder.make_requests(ctx, ad_campaign, &ad_source).await?;
        let call_info = self.do_request(http_client, ctx, ad_campaign, &ad_source, request_data).await?;
        let bid = self.bidder.make_bids(ctx, ad_campaign, &call_info.response_data).await?;
        // todo!()
        Ok(BidderResponse { ad_campaign: todo!(), ad_source, bid_response: todo!() })
    }

    async fn do_request(&self, http_client: &Client, ctx: &AdxContext<'_>, ad_campaign: &AdCampaign, ad_source: &AdSource, request_data: HttpRequestData) -> Result<HttpCallInfo> {
        let response = http_client.request(request_data.method, request_data.uri)
            .timeout(REQUEST_TIMEOUT)
            .body(request_data.body).send().await;


        response.err();

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