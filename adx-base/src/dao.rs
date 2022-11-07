use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use anyhow::{Context, Result};

use crate::{AdSource, Media};
use crate::errors::*;
use crate::indexer::*;
use crate::types::{AdCampaign, Adslot};

#[derive(Debug)]
pub struct AdxDB {
    indexer: Indexer,

    adslot_map: HashMap<u64, Adslot>,
    media_map: HashMap<u64, Media>,
}

impl AdxDB {
    pub fn new(indexer: Indexer) -> AdxDB {
        AdxDB { indexer, adslot_map: HashMap::default(), media_map: HashMap::new() }
    }


    pub fn get_adslot_by_id(&self, id: u64) -> Result<&Adslot> {
        return self.adslot_map.get(&id).context("media not found");
    }

    pub fn get_media_by_id(&self, id: u64) -> Result<&Media> {
        return self.media_map.get(&id).context("media not found");
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