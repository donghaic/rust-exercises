use anyhow::Result;
use async_trait::async_trait;
use enum_dispatch::*;

use crate::types::{AdxContext, BidderResponse};

#[async_trait]
#[enum_dispatch(Validators)]
pub trait BidValidator {
    async fn is_ok(&self, ctx: &mut AdxContext<'_>, bid: &BidderResponse) -> bool;
}


pub struct BidValidatorChain {
    validators: Vec<Validators>,
}

impl BidValidatorChain {
    pub fn new() -> Self {
        let mut validators = Self { validators: Vec::default() };
        validators.add(BidFloorValidator{}.into());
        return validators;
    }

    fn add(&mut self, validator: Validators) {
        self.validators.push(validator);
    }

    pub async fn do_validator(&self, ctx: &mut AdxContext<'_>, bid_responses: Vec<BidderResponse>) -> Result<Vec<BidderResponse>> {
        let mut matched_bids = Vec::new();
        for bid_response in bid_responses {
            let mut is_ok = true;
            for validator in self.validators.iter() {
                is_ok = validator.is_ok(ctx, &bid_response).await;
                if !is_ok {
                    break;
                }
            }
            if is_ok {
                matched_bids.push(bid_response);
            }
        }

        Ok(matched_bids)
    }
}


#[enum_dispatch]
enum Validators {
    BidFloorValidator,
}

// -----------------------------------------------------------
pub struct BidFloorValidator {}

impl BidFloorValidator {}


#[async_trait]
impl BidValidator for BidFloorValidator {
    async fn is_ok(&self, ctx: &mut AdxContext<'_>, bid: &BidderResponse) -> bool {
        ctx.req_id = "".to_string();

        return true;
    }
}


