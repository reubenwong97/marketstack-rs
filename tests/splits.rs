use chrono::NaiveDate;

use marketstack::api::{splits, AsyncQuery, Query};
use marketstack::{AsyncMarketstack, Marketstack, SplitsData};

mod setup;

#[test]
#[ignore]
fn test_splits() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = splits::Splits::builder()
        .symbol("AAPL")
        .limit(3)
        .unwrap()
        .build()
        .unwrap();
    let splits_result: SplitsData = endpoint.query(&client).unwrap();

    assert_eq!(splits_result.pagination.limit, 3);
    assert_eq!(splits_result.pagination.offset, 0);

    assert_eq!(splits_result.data.len(), 3);
}

#[tokio::test]
#[ignore]
async fn test_async_splits() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = splits::Splits::builder()
        .limit(3)
        .unwrap()
        .symbol("AAPL")
        .build()
        .unwrap();
    let eod_result: SplitsData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(eod_result.pagination.limit, 3);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 3);
}

#[test]
#[ignore]
fn test_splits_date() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = splits::Splits::builder()
        .symbol("AAPL")
        .date_from(NaiveDate::from_ymd_opt(2020, 8, 29).unwrap())
        .date_to(NaiveDate::from_ymd_opt(2020, 9, 2).unwrap())
        .build()
        .unwrap();
    let splits_result: SplitsData = endpoint.query(&client).unwrap();

    assert_eq!(splits_result.data.len(), 1);
    assert_eq!(splits_result.data[0].split_factor, 4.0);
}
