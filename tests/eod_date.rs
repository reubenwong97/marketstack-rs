//! [WARNING] Code is this module is untested due to API plan restrictions.

use chrono::NaiveDate;
use marketstack::api::common::SortOrder;
use marketstack::api::eod::EodDate;
use marketstack::api::{AsyncQuery, Query};
use marketstack::{AsyncMarketstack, EodData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_eod_date() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = EodDate::builder()
        .date(NaiveDate::from_ymd_opt(2022, 4, 23).unwrap())
        .symbol("AAPL")
        .build()
        .unwrap();
    let eod_result: EodData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 100);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 1);
}

#[test]
#[ignore]
fn test_eod_date_paged() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = EodDate::builder()
        .date(NaiveDate::from_ymd_opt(2022, 4, 23).unwrap())
        .symbol("AAPL")
        .limit(5)
        .unwrap()
        .build()
        .unwrap();
    let eod_result: EodData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 5);
    assert_eq!(eod_result.data.len(), 1);
}

#[test]
#[ignore]
fn test_eod_date_sorting() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = EodDate::builder()
        .symbol("AAPL")
        .date(NaiveDate::from_ymd_opt(2022, 4, 23).unwrap())
        .sort(SortOrder::Ascending)
        .build()
        .unwrap();

    let _: EodData = endpoint.query(&client).unwrap();
}

#[tokio::test]
#[ignore]
async fn test_async_eod_date() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = EodDate::builder()
        .date(NaiveDate::from_ymd_opt(2022, 4, 23).unwrap())
        .symbol("AAPL")
        .build()
        .unwrap();
    let eod_result: EodData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(eod_result.pagination.limit, 100);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 1);
}

#[tokio::test]
#[ignore]
async fn test_async_eod_date_paged() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = EodDate::builder()
        .symbol("AAPL")
        .date(NaiveDate::from_ymd_opt(2022, 4, 23).unwrap())
        .limit(5)
        .unwrap()
        .build()
        .unwrap();
    let eod_result: EodData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(eod_result.pagination.limit, 5);
    assert_eq!(eod_result.data.len(), 1);
}
