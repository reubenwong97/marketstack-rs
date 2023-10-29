use marketstack::api::common::SortOrder;
use marketstack::api::{eod, tickers, AsyncQuery, Query};
use marketstack::{AsyncMarketstack, EodDataItem, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_tickers_eod() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = tickers::Tickers::builder()
        .ticker("AAPL")
        .eod(eod::Eod::builder().build().unwrap())
        .build()
        .unwrap();
    let eod_result: EodDataItem = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.symbol, "AAPL");
}
