use std::vec;

use anyhow::Result;
use async_trait::async_trait;

use crate::{AdCampaign, AdxContext};

#[async_trait]
pub trait AdFilter {
    async fn is_ok(&self, ctx: &mut AdxContext, ad_campaign: &AdCampaign) -> bool;
}


pub struct AdFilterChain {
    simple_filters: Vec<Box<dyn AdFilter>>,
}

impl AdFilterChain {
    pub fn new() -> Self {
        let mut simple_filters: Vec<Box<dyn AdFilter>> = Vec::new();
        simple_filters.push(Box::new(DeviceIdFilter::new()));
        simple_filters.push(Box::new(PacingFilter::new()));


        Self { simple_filters }
    }

    pub async fn do_filter(&self, ctx: &mut AdxContext<'_>, ad_campaigns: &Vec<AdCampaign>) -> Result<Vec<AdCampaign>> {
        let mut filtered_campaigns = vec![];
        let mut is_match = true;
        for ad_campaign in ad_campaigns {
            for filter in self.simple_filters.iter() {
                is_match = filter.is_ok(ctx, &ad_campaign).await;
                if !is_match {
                    break;
                }
            }

            if is_match {
                filtered_campaigns.push(ad_campaign);
            }
        }

        Ok(vec![])
    }
}


// --------------------------------------------------------------------

pub struct DeviceIdFilter {}

impl DeviceIdFilter {
    pub fn new() -> DeviceIdFilter {
        DeviceIdFilter {}
    }
}

#[async_trait]
impl AdFilter for DeviceIdFilter {
    async fn is_ok(&self, ctx: &mut AdxContext, ad_campaign: &AdCampaign) -> bool {
        let is_ok = ad_campaign.is_device_id_whitelist(ctx.bid_request.device_id.as_str());
        if !is_ok {
            //ctx.err_code.borrow_mut().insert(1, "DeviceIdFilter".to_string());
        }
        is_ok
    }
}


pub struct PacingFilter {}

impl PacingFilter {
    fn new() -> PacingFilter {
        PacingFilter {}
    }
}

#[async_trait]
impl AdFilter for PacingFilter {
    async fn is_ok(&self, ctx: &mut AdxContext, ad_campaign: &AdCampaign) -> bool {
        return ad_campaign.sale_num < 100;
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::sync::{Arc, RwLock};

    use crate::types::*;

    use super::*;

    #[tokio::test]
    async fn test_ad_filters() {
        let mut ctx = AdxContext {
            req_id: "".to_string(),
            ad_slot: &Adslot { id: 0 },
            media: &Media { id: 0 },
            bid_request: BidRequest { id: "".to_string(), device_id: "".to_string() },
            // err_code: RefCell::new(RwLock::new(Default::default())),
        };

        let ad_campaign = AdCampaign {
            id: 0,
            device_id_whitelist: "".to_string(),
            sale_num: 0,
            _device_id_whitelist: Default::default(),
            ad_source_id: 1,
        };

        let mut simple_filters: Vec<Box<dyn AdFilter>> = Vec::new();
        simple_filters.push(Box::new(DeviceIdFilter::new()));
        simple_filters.push(Box::new(PacingFilter::new()));

        for adfilter in simple_filters.iter() {
            let is_ok = adfilter.is_ok(&mut ctx, &ad_campaign).await;
            println!("111 is ok = {}", is_ok)
        }
        println!("end")
    }
}