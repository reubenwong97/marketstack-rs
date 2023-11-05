use chrono::NaiveDate;

use marketstack::api::common::Interval;
use marketstack::api::intraday::Intraday;
use marketstack::api::Query;
use marketstack::{IntradayData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_intraday() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Intraday::builder().symbol("AAPL").build().unwrap();
    let intraday_result: IntradayData = endpoint.query(&client).unwrap();

    assert_eq!(intraday_result.pagination.limit, 100);
    assert_eq!(intraday_result.pagination.offset, 0);

    assert_eq!(intraday_result.data.len(), 100);
    assert!(intraday_result.data.iter().all(|x| x.symbol == "AAPL"));
}

#[test]
#[ignore]
fn test_intraday_paged() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Intraday::builder()
        .symbol("AAPL")
        .limit(5)
        .unwrap()
        .build()
        .unwrap();
    let intraday_result: IntradayData = endpoint.query(&client).unwrap();

    assert_eq!(intraday_result.pagination.limit, 5);
    assert_eq!(intraday_result.data.len(), 5);
    assert!(intraday_result.data.iter().all(|x| x.symbol == "AAPL"));
}

#[test]
#[ignore]
fn test_intraday_latest() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Intraday::builder()
        .symbol("AAPL")
        .latest(true)
        .build()
        .unwrap();
    let intraday_result: IntradayData = endpoint.query(&client).unwrap();

    assert_eq!(intraday_result.pagination.limit, 100);
    assert_eq!(intraday_result.pagination.offset, 0);

    assert_eq!(intraday_result.data.len(), 1);
    assert!(intraday_result.data.iter().all(|x| x.symbol == "AAPL"));
}

#[test]
#[ignore]
fn test_intraday_date() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Intraday::builder()
        .symbol("AAPL")
        .date(NaiveDate::from_ymd_opt(2023, 10, 30).unwrap())
        .build()
        .unwrap();
    let intraday_result: IntradayData = endpoint.query(&client).unwrap();

    assert_eq!(intraday_result.pagination.limit, 100);
    assert_eq!(intraday_result.pagination.offset, 0);
    assert_eq!(intraday_result.pagination.count, 7);
    assert_eq!(intraday_result.pagination.total, 7);

    assert_eq!(intraday_result.data.len(), 7);
    assert!(intraday_result.data.iter().all(|x| x.symbol == "AAPL"));
}

#[test]
#[ignore]
fn test_intraday_date_interval() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Intraday::builder()
        .symbol("AAPL")
        .date(NaiveDate::from_ymd_opt(2023, 10, 30).unwrap())
        .interval(Interval::ThirtyMinutes)
        .build()
        .unwrap();
    let intraday_result: IntradayData = endpoint.query(&client).unwrap();

    assert_eq!(intraday_result.pagination.limit, 100);
    assert_eq!(intraday_result.pagination.offset, 0);
    assert_eq!(intraday_result.pagination.count, 14);
    assert_eq!(intraday_result.pagination.total, 14);

    assert_eq!(intraday_result.data.len(), 14);
    assert!(intraday_result.data.iter().all(|x| x.symbol == "AAPL"));
}
