use std::collections::HashMap;

use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;

use crate::{dao::AdxDB, types::*};
use crate::dispatcher::*;
use crate::filters::AdFilterChain;

pub struct Exchange {
    adx_db: AdxDB,
    ad_filters: AdFilterChain,
    bidders: HashMap<u64, BidderAdapter>,
}

impl Exchange {
    pub async fn auction(&self, ctx: &AdxContext<'_>) -> Result<AuctionResult> {
        let ad_campaigns = self.adx_db.get_campaigns(ctx.ad_slot);
        let filtered_campaigns = self.ad_filters.do_filter(ctx, &ad_campaigns).await;
        let bid_res = self.get_bids(ctx, &ad_campaigns).await;
        panic!()
    }

    pub async fn get_bids(&self, ctx: &AdxContext<'_>, ad_campaigns: &Vec<AdCampaign>) -> Result<Vec<BidderResponse>> {
        let client = Client::new();
        let mut bid_response_fut = vec![];
        for ad_campaign in ad_campaigns.iter() {
            let ad_source = self.adx_db.get_ad_source_by_id(ad_campaign.ad_source_id);
            let bidder = self.bidders.get(&ad_source.id).unwrap_or(self.bidders.get(&DefaultBidderId).unwrap());
            let res = bidder.request_bid(&client, ctx, ad_campaign, ad_source);
            bid_response_fut.push(res);
        }

        let bid_responses = join_all(bid_response_fut).await;

        let mut result = vec![];
        for res in bid_responses {
            result.push(res.unwrap())
        }
        Ok(result)
    }
}