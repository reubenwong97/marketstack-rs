use marketstack::api::{self, eod, Query};
use marketstack::{EodData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_eod() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = eod::Eod::builder().symbol("AAPL").build().unwrap();
    let eod_result: EodData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 100);
    assert_eq!(eod_result.pagination.offset, 0);
}

#[test]
#[ignore]
fn test_eod_paged() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let pageable_endpoint = eod::Eod::builder().symbol("AAPL").build().unwrap();
    let first_200: EodData = api::paged(pageable_endpoint, api::Pagination::Limit(200))
        .query(&client)
        .unwrap();
}
