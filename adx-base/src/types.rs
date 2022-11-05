use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct AdSource {
    pub id: u64,
    pub uri: String,
}

pub struct HttpResponseData {
    pub status_code: u8,
    pub body: Vec<u8>,
}

pub struct HttpRequestData {
    pub method: String,
    pub uri: String,
    pub body: Vec<u8>,
}


pub struct Adslot {
    pub id: u64,
}


pub struct Media {
    pub id: u64,
}

#[derive(Debug, Clone)]
pub struct AdCampaign {
    pub id: u64,
    pub device_id_whitelist: String,
    pub sale_num: u32,
    pub ad_source_id: u64,

    pub _device_id_whitelist: HashSet<String>,
}

impl AdCampaign {
    pub fn is_device_id_whitelist(&self, device_id: &str) -> bool {
        return self._device_id_whitelist.contains(device_id);
    }
}

pub struct AdxContext<'a> {
    pub req_id: String,
    pub ad_slot: &'a Adslot,
    pub media: &'a Media,
    pub bid_request: BidRequest,

   // pub err_code: RefCell<RwLock<HashMap<u64, String>>>,
}

pub struct BidResponse {
    pub id: String,
}

pub struct BidRequest {
    pub id: String,
    pub device_id: String,
}

pub struct BidderResponse {}

pub struct HttpCallInfo {
    pub response_data: HttpResponseData,
}

pub struct AuctionResult {}