use marketstack::api::common::SortOrder;
use marketstack::api::{eod, tickers, AsyncQuery, Query};
use marketstack::{AsyncMarketstack, Marketstack, TickersData};

mod setup;

#[test]
#[ignore]
fn test_tickers() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = tickers::Tickers::builder()
        .limit(3)
        .unwrap()
        .build()
        .unwrap();
    let tickers_result: TickersData = endpoint.query(&client).unwrap();

    assert_eq!(tickers_result.pagination.limit, 3);
}
