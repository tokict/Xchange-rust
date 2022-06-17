use chrono::{DateTime, Utc};
pub struct ConstructorParams {
    ask_fees: Vec<Fee>,
    bid_fees: Vec<Fee>,
    trade_fee: Fee,
    margin_fees: Vec<MarginFee>,
    treasury_wallet: Vec<near_sdk::AccountId>,
    escrow_wallet: Vec<near_sdk::AccountId>,
    resources: Vec<Resource>,

    sellers: Vec<near_sdk::AccountId>,
    buyers: Vec<near_sdk::AccountId>,
}

/*
 * This is the basic resource
 */
pub struct Resource {
    id: u16,
    name: String,
    symbol: String,
    measurement_unit: String,
}

/*
 * This is the resource for sale from one user
 * 1 to many
 */
pub struct ResourceAsk {
    id: u16,
    resource_id: u16,
    asker: near_sdk::AccountId,
    min_units: u32,
    max_units: u32,
    ask_ppu: u128,
    applied_fee_ids: [u16],
}

/*
 * This is the resource bid from one user
 * 1 to many
 */
pub struct ResourceBid {
    id: u16,

    bidder: near_sdk::AccountId,
    resource_id: u16,
    min_units: u32,
    max_units: u32,
    bid_ppu: u128,
    applied_fee_ids: Vec<u16>,
    margin_fee_id: Vec<u16>,
}

// This is what we generate when we get payment. We should not have any ghost incoming payments in the system
pub struct IncomingTradePayment {
    transaction_time: DateTime<Utc>,
    buyer: near_sdk::AccountId,
    near_transferred: u128,
    trade_offer_id: u128,
    applied_fee: u16,
    fee_amount: u128,
}

// This is what we generate when we release payment
pub struct OutgoingPayment {
    trade_offer_id: u16,
    user: near_sdk::AccountId,
    transaction_time: u128,
    near_transferred: u128,
    is_seller_payment: bool,
    is_refund: bool,
    is_margin: bool,
}

// All fees including margin
pub struct Fee {
    id: u16,
    percentage: u16,
    amount: u128,
    fee_type: String,
    cumulative: bool,
}

// All fees including margin
pub struct MarginFee {
    id: u16,
    percent_over_median: u16,
    percentage: u16,
}

// Trade offer is a time limited matching on price and quantity between a buyer and seller;

pub struct TradeOffer {
    id: u128,
    period_id: u16,
    resource_id: u16,
    units: u32,
    seller: near_sdk::AccountId,
    buyer: near_sdk::AccountId,
    expires_at: u128,
    accepted_at: u128,
}
