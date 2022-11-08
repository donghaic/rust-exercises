use std::{sync::Arc, time::Duration};

use moka::sync::Cache;

#[derive(Debug)]
pub struct AdCampaign {
    id: u64,
    name: String,
    price: u64,
}

pub struct Dal {
    campaign_cache: Cache<u64, Arc<AdCampaign>>,
}

impl Dal {
    pub fn new() -> Self {
        let cache = Cache::builder()
            // Time to live (TTL): 3 seconds
            .time_to_live(Duration::from_secs(3))
            // Create the cache.
            .build();

        Dal {
            campaign_cache: cache,
        }
    }

    pub fn add_campaign(&mut self, campaign: AdCampaign) {
        self.campaign_cache.insert(campaign.id, Arc::new(campaign));
    }

    pub fn get_campaign(&self, id: u64) -> Option<Arc<AdCampaign>> {
        self.campaign_cache.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn test_moka() {
        let mut dal = Dal::new();
        let campaign = AdCampaign {
            id: 1,
            name: "广告1".to_string(),
            price: 10,
        };

        dal.add_campaign(campaign);

        let camp = dal.get_campaign(1);
        assert!(camp.is_some());
        println!("{:#?}", camp);

        let campaign = AdCampaign {
            id: 1,
            name: "广告2".to_string(),
            price: 10,
        };

        dal.add_campaign(campaign);
        let camp = dal.get_campaign(1);
        assert!(camp.is_some());
        println!("{:#?}", camp);

        thread::sleep(Duration::from_secs(4));

        let camp = dal.get_campaign(1);
        assert!(camp.is_none());
        println!("{:#?}", camp);
    }
}
