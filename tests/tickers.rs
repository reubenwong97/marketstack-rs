use chrono::NaiveDate;
use marketstack::api::common::SortOrder;
use marketstack::api::dividends::Dividends;
use marketstack::api::splits::Splits;
use marketstack::api::{eod, tickers, Query};
use marketstack::{
    DividendsData, EodDataItem, Marketstack, SplitsData, TickersData, TickersEodData,
};

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

#[test]
#[ignore]
fn test_tickers_eod_latest() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = tickers::Tickers::builder()
        .ticker("AAPL")
        .eod(eod::Eod::builder().latest(true).build().unwrap())
        .build()
        .unwrap();

    let tickers_eod_latest_result: EodDataItem = endpoint.query(&client).unwrap();

    assert_eq!(tickers_eod_latest_result.symbol, "AAPL");
}

#[test]
#[ignore]
fn test_tickers_eod_date() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = tickers::Tickers::builder()
        .ticker("AAPL")
        .eod(
            eod::Eod::builder()
                .date(NaiveDate::from_ymd_opt(2023, 10, 25).unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let tickers_eod_date_result: EodDataItem = endpoint.query(&client).unwrap();

    assert_eq!(tickers_eod_date_result.symbol, "AAPL");
}

#[test]
#[ignore]
fn test_tickers_splits() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = tickers::Tickers::builder()
        .ticker("AAPL")
        .splits(Splits::builder().build().unwrap())
        .build()
        .unwrap();

    let tickers_splits_result: SplitsData = endpoint.query(&client).unwrap();

    assert_eq!(tickers_splits_result.data[0].symbol, "AAPL")
}

#[test]
#[ignore]
fn test_tickers_dividends() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = tickers::Tickers::builder()
        .ticker("AAPL")
        .dividends(Dividends::builder().build().unwrap())
        .build()
        .unwrap();

    let tickers_dividends_result: DividendsData = endpoint.query(&client).unwrap();

    assert_eq!(tickers_dividends_result.data[0].symbol, "AAPL")
}
