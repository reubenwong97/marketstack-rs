use marketstack::api::common::SortOrder;
use marketstack::api::{eod, tickers, Query};
use marketstack::{Marketstack, TickersData, TickersEodData};

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

#[test]
#[ignore]
fn test_tickers_eod() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = tickers::Tickers::builder()
        .ticker("AAPL")
        .eod(
            eod::Eod::builder()
                .sort(SortOrder::Ascending)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let tickers_eod_result: TickersEodData = endpoint.query(&client).unwrap();

    assert_eq!(tickers_eod_result.data.eod[0].symbol, "AAPL");
}
