use std::collections::HashSet;

use crate::AdSource;
use crate::types::{AdCampaign, Adslot};
use crate::indexer::*;

#[derive(Debug, Clone)]
pub struct AdxDB {
    indexer : Indexer,
}

impl AdxDB {
    pub fn new(indexer : Indexer) -> AdxDB {
        AdxDB {indexer}
    }

    pub fn get_campaigns(&self, adslot: &Adslot) -> Vec<AdCampaign> {
        let mut whitelist = HashSet::new();
        whitelist.insert("value".to_string());
        let mut res = vec![];
        res.push(AdCampaign { id: 1, device_id_whitelist: "111".into(), sale_num: 20, _device_id_whitelist: whitelist, ad_source_id: 1 });

        return res;
    }

    pub fn get_ad_source_by_id(&self, id: u64) -> AdSource {
        AdSource { id, uri: "".to_string() }
    }
}