use chrono::NaiveDate;

use marketstack::api::{dividends, AsyncQuery, Query};
use marketstack::{AsyncMarketstack, DividendsData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_dividends() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = dividends::Dividends::builder()
        .symbol("AAPL")
        .limit(5)
        .unwrap()
        .build()
        .unwrap();
    let eod_result: DividendsData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 5);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 5);
}

#[tokio::test]
#[ignore]
async fn test_async_dividends() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = dividends::Dividends::builder()
        .limit(3)
        .unwrap()
        .symbol("AAPL")
        .build()
        .unwrap();
    let eod_result: DividendsData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(eod_result.pagination.limit, 3);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 3);
}

#[test]
#[ignore]
fn test_dividends_date() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = dividends::Dividends::builder()
        .symbol("AAPL")
        .date_from(NaiveDate::from_ymd_opt(2023, 8, 10).unwrap())
        .date_to(NaiveDate::from_ymd_opt(2023, 8, 12).unwrap())
        .build()
        .unwrap();
    let dividends_result: DividendsData = endpoint.query(&client).unwrap();

    assert_eq!(dividends_result.data.len(), 1);
    assert_eq!(dividends_result.data[0].dividend, 0.24);
}
