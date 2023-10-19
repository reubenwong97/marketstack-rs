use marketstack::api::common::SortOrder;
use marketstack::api::eod;
use marketstack::api::{AsyncQuery, Query};
use marketstack::{AsyncMarketstack, EodData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_eod_latest() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = eod::EodLatest::builder().symbol("AAPL").build().unwrap();
    let eod_result: EodData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 100);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 1);
}
