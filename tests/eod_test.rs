use marketstack::api::{eod, Query};
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

    assert_eq!(eod_result.data.len(), 100);
}

#[test]
#[ignore]
fn test_eod_paged() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = eod::Eod::builder()
        .symbol("AAPL")
        .limit(5)
        .unwrap()
        .build()
        .unwrap();
    let eod_result: EodData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 5);
    assert_eq!(eod_result.data.len(), 5);
}
